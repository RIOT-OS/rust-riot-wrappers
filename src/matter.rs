use crate::mutex::Mutex;
use crate::socket_embedded_nal_async_udp::UnconnectedUdpSocket;
use crate::{vfs, ztimer};
use crate::random::Random;
use crate::error::NumericError;
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
use rs_matter::transport::network::{UdpReceive, UdpSend};

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
pub struct VfsDataFetcher;

impl CommissioningDataFetcher for VfsDataFetcher {
    fn read_commissioning_data(&self) -> Result<(u16, u32), NumericError> {
        // TODO: Read Commissioning Data from VFS
        todo!("not implemented - see https://github.com/RIOT-OS/rust-riot-wrappers/issues/93");

        let mut passcode_data: [u8; 4] = [0; 4];
        let mut passcode_file = vfs::File::open("/const/passcode")?;
        passcode_file.read(&mut passcode_data)?;
        let passcode = u32::from_be_bytes(passcode_data);

        let mut discriminator_data: [u8; 2]= [0; 2];
        let mut discriminator_file = vfs::File::open("/const/discriminator")?;
        discriminator_file.read(&mut discriminator_data)?;
        let discriminator = u16::from_be_bytes(discriminator_data);

        Ok((discriminator, passcode))
    }
}

impl DevAttDataFetcher for VfsDataFetcher {
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