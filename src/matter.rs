use crate::println;
use crate::mutex::Mutex;
use crate::socket_embedded_nal_async_udp::UnconnectedUdpSocket;
use crate::ztimer;

use embedded_nal_async::{SocketAddr, UnconnectedUdp};
use embassy_futures::select::{Either, select};
use embedded_hal_async::delay::DelayNs as _;
use embassy_sync::{
    signal::Signal,
    blocking_mutex::raw::NoopRawMutex,
};
use rs_matter::error::{Error, ErrorCode};
use rs_matter::transport::network::{UdpReceive, UdpSend};
use log::{debug, warn, error, Level, LevelFilter, Log, Record, SetLoggerError};

pub struct MatterCompatUdpSocket {
    local_addr: SocketAddr,
    socket: Mutex<UnconnectedUdpSocket>,
    release_socket_notification: Notification,
    socket_released_notification: Notification,
}

struct RiotLogger;

pub type Notification = Signal<NoopRawMutex, ()>;

static LOGGER: RiotLogger = RiotLogger;

impl Log for RiotLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() >= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("[{}] {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

pub fn init_logger(level: LevelFilter) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)
        .map(|_| log::set_max_level(level))
}

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
                    match res {
                        Ok((bytes_recvd, local_addr, remote_addr)) => {
                            if remote_addr.is_ipv4() {
                                // IPv4 not supported!
                                return Ok((bytes_recvd, remote_addr));
                            }
                        }
                        Err(_) => { error!("Error during UDP receive!"); }
                    }
                    // return receive result
                    let (bytes_recvd, remote_addr) = res.map(|(bytes_recvd, _, remote_addr)|
                        (bytes_recvd, remote_addr)
                    ).map_err(|_| Error::new(ErrorCode::StdIoError))?;
                    return Ok((bytes_recvd, remote_addr));
                }
            }
        }
    }
}
