use core::time::Duration;
use crate::mutex::Mutex;
use crate::socket_embedded_nal_async_udp::UnconnectedUdpSocket;
use crate::{vfs, ztimer};
use crate::random::Random;
use crate::error::NumericError;
use riot_sys::{ZTIMER_MSEC, inline::{ztimer_now, ztimer_clock}};
use embedded_nal_async::{UnconnectedUdp, SocketAddr};
use embassy_futures::select::{Either, select};
use embedded_hal_async::delay::DelayNs as _;
use embassy_sync::{
    signal::Signal,
    blocking_mutex::raw::NoopRawMutex,
};
use rand_core_06::RngCore as _;
use rs_matter::error::{Error, ErrorCode};
use rs_matter::data_model::sdm::dev_att::{DataType, DevAttDataFetcher};
use rs_matter::Matter;
use rs_matter::transport::network::{UdpReceive, UdpSend};

// RIOT_EPOCH(2020) in seconds since UNIX Epoch
const RIOT_EPOCH_SECS: u64 = 1577833200;

pub struct MatterCompatUdpSocket {
    local_addr: SocketAddr,
    socket: Mutex<UnconnectedUdpSocket>,
    release_socket_notification: Notification,
    socket_released_notification: Notification,
}

pub type Notification = Signal<NoopRawMutex, ()>;

impl MatterCompatUdpSocket {
    pub fn new(local_addr: SocketAddr, socket: UnconnectedUdpSocket) -> Self {
        Self {
            local_addr,
            socket: Mutex::new(socket),
            release_socket_notification: Notification::new(),
            socket_released_notification: Notification::new(),
        }
    }
}

impl UdpSend for &MatterCompatUdpSocket {
    async fn send_to(&mut self, data: &[u8], addr: SocketAddr) -> Result<(), Error> {
        if addr.is_ipv4() {
            // IPv4 not supported!
            return Ok(());
        }
        // Tell recv_from to release mutex
        self.release_socket_notification.signal(());
        ztimer::Delay.delay_ms(10).await;
        let mut sock = self.socket.try_lock().expect("receiver should have ensured that this mutex is free");
        sock.send(self.local_addr, addr, data)
            .await
            .map_err(|_| Error::new(ErrorCode::StdIoError))?;
        // Release socket and notify recv_from -> sending is finished
        drop(sock);
        self.socket_released_notification.signal(());
        Ok(())
    }
}

impl UdpReceive for &MatterCompatUdpSocket {
    async fn recv_from(&mut self, buffer: &mut [u8]) -> Result<(usize, SocketAddr), Error> {
        loop {
            let mut sock = self.socket.try_lock().expect("sender should have ensured that this mutex is free");
            match select(
                self.release_socket_notification.wait(),
                sock.receive_into(buffer),
            ).await {
                Either::First(_) => {
                    // Release Mutex for send_to
                    drop(sock);
                    // ... and wait until available again
                    self.socket_released_notification.wait().await;
                    continue;
                }
                Either::Second(res) => {
                    let (bytes_recvd, remote_addr) = res.map(|(bytes_recvd, _, remote_addr)|
                        (bytes_recvd, remote_addr)
                    ).map_err(|_| Error::new(ErrorCode::StdIoError))?;
                    return Ok((bytes_recvd, remote_addr));
                }
            }
        }
    }
}

/// Generate random bytes using the RIOT RNG
#[cfg(riot_module_random)]
pub fn sys_rand(buf: &mut [u8]) {
    Random::new().fill_bytes(buf);
}

/// Returns Duration since UNIX Epoch (01.01.1970 00:00:00)
pub fn sys_epoch() -> Duration {
    let epoch_sec = RIOT_EPOCH_SECS + get_riot_sec() as u64;
    let mut epoch_ms: u32 = unsafe {
        ztimer_now(ZTIMER_MSEC as *mut ztimer_clock) % 1000
    };
    Duration::from_millis((epoch_sec*1000) + epoch_ms as u64)
}

/// Returns elapsed seconds since `RIOT_EPOCH` using `periph_rtc`
#[cfg(riot_module_periph_rtc)]
fn get_riot_sec() -> u32 {
    use riot_sys::{rtc_get_time, rtc_tm_normalize, rtc_mktime};
    unsafe {
        let mut now = riot_sys::tm::default();
        rtc_get_time(&mut now);
        rtc_tm_normalize(&mut now);
        rtc_mktime(&mut now)
    }
}

/// Returns elapsed seconds since `RIOT_EPOCH` using `ztimer_sec`
#[cfg(all(not(riot_module_periph_rtc), riot_module_ztimer_sec))]
fn get_riot_sec() -> u32 {
    use riot_sys::ZTIMER_SEC;
    unsafe {
        ztimer_now(ZTIMER_SEC as *mut ztimer_clock)
    }
}

pub trait CommissioningDataFetcher {
    /// Reads Discriminator and Passcode) from device firmware
    fn read_commissioning_data(&self) -> Result<(u16, u32), NumericError>;
}

/// Provides methods for reading Matter data from VFS (Virtual File System Layer)
/// such as commissioning and device attestation data.
/// For now, only reading from constfs (mountpoint `/const`) is supported, but could be extended to support various filesystems and proper encryption.
///
/// It is expected that the following files exist in the VFS layer of the target device, otherwise it will panic:
/// - `/const/cd`: Certificate Declaration
/// - `/const/pai`: Product Attestation Intermediary Certificate
/// - `/const/dac`: Device Attestation Certificate
/// - `/const/dac_pubkey`: DAC Public Key
/// - `/const/dac_privkey`: DAC Private Key
/// - `/const/passcode`: Passcode required for successful commissioning
/// - `/const/discriminator`: Required for Node Discovery via mDNS
pub struct FirmwareDataFetcher;

impl FirmwareDataFetcher {
    pub fn new() -> Self{
        Self {}
    }
}

impl CommissioningDataFetcher for FirmwareDataFetcher {
    fn read_commissioning_data(&self) -> Result<(u16, u32), NumericError> {
        // TODO: Read Commissioning Data from VFS
        todo!("not implemented - see https://github.com/RIOT-OS/rust-riot-wrappers/issues/93");

        let mut passcode_data: [u8; 4] = [0; 4];
        let mut passcode_file = vfs::File::open("/const/passcode")?;
        passcode_file.read(&mut passcode_data)?;
        let passcode = u32::from_le_bytes(passcode_data);

        let mut discriminator_data: [u8; 2]= [0; 2];
        let mut discriminator_file = vfs::File::open("/const/discriminator")?;
        discriminator_file.read(&mut discriminator_data)?;
        let discriminator = u16::from_le_bytes(discriminator_data);

        Ok((discriminator, passcode))
    }
}

impl DevAttDataFetcher for FirmwareDataFetcher {
    fn get_devatt_data(&self, data_type: DataType, data: &mut [u8]) -> Result<usize, Error> {
        // TODO: Read Device Attestation Data from VFS
        todo!("not implemented - see https://github.com/RIOT-OS/rust-riot-wrappers/issues/93");
        let src_path = match data_type {
                DataType::CertDeclaration => "/const/cd",
                DataType::PAI => "/const/pai",
                DataType::DAC => "/const/dac",
                DataType::DACPubKey => "/const/dac_pubkey",
                DataType::DACPrivKey => "/const/dac_privkey"
        };
        let mut src = vfs::File::open(src_path).map_err(|_| Error::new(ErrorCode::StdIoError))?;
        let len = src.read(data)
            .map_err(|_| Error::new(ErrorCode::StdIoError))?;
        if len <= data.len() {
            let data = &mut data[0..len];
            Ok(len)
        } else {
            Err(ErrorCode::NoSpace.into())
        }
    }
}

/// Used for storing and loading ACL and Fabric data in a key-value store
pub struct PersistenceManager<'a> {
    matter: &'a Matter<'a>,
    buf: [u8; 4096],
    dir: &'a str,
}

impl<'a> PersistenceManager<'a> {
    #[inline(always)]
    pub fn new(matter: &'a Matter<'a>) -> Result<Self, Error> {
        let mut buf = [0; 4096];
        Ok(Self { matter, buf, dir: "data" })
    }

    /// Waits for data changes by the Matter service and saves ACL and/or fabric data
    pub async fn run(&mut self) -> Result<(), Error> {
        todo!("A key-value store must be implemented for PersistenceManager");
        loop {
            self.matter.wait_changed().await;
            if self.matter.is_changed() {
                if let Some(data) = self.matter.store_acls(&mut self.buf)? {
                    todo!("not implemented yet")
                    //Self::store("acls", data)?;
                }

                if let Some(data) = self.matter.store_fabrics(&mut self.buf)? {
                    todo!("not implemented yet")
                    //Self::store("fabrics", data)?;
                }
            }
        }
    }

    /// Loads data from the key-value store and writes it to the buffer
    fn load<'b>(key: &str, buf: &'b mut [u8]) -> Result<Option<&'b [u8]>, Error> {
        todo!("not implemented yet")
    }

    /// Stores data to the key-value store
    fn store(key: &str, data: &[u8]) -> Result<(), Error> {
        todo!("not implemented yet")
    }
}
