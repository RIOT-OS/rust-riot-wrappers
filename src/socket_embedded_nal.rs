//! An implementation of the [embedded-nal] (Network Abstradtion Layer) UDP trait based on RIOT
//! sockets
//!
//! [embedded-nal]: https://docs.rs/embedded-nal/0.1.0/embedded_nal/

use core::mem::MaybeUninit;

use crate::error::NegativeErrorExt;

/// The operating system's network stack, implementing ``embedded_nal::UdpStack``.
///
/// This is accessed using the static ``STACK`` instance.
///
/// The stack can be cloned, as it is not a resource that needs any synchronization. This is not
/// made implicit as Copy, though (although there's not technical reason not to). That is to alert
/// users to the difficulties that'd arise when taking ownership of a stack rather than just using
/// it through a shared reference (which is generally possible in ``embedded_nal``).
#[derive(Clone)]
pub struct Stack {
    _private: (),
}

pub static STACK: Stack = Stack { _private: () };

pub struct UdpSocket {
    socket: riot_sys::sock_udp_t,
    timeout_us: u32,
}

impl embedded_nal::UdpStack for Stack {
    type UdpSocket = UdpSocket;
    type Error = crate::error::NumericError;

    fn open(
        &self,
        remote: embedded_nal::SocketAddr,
        mode: embedded_nal::Mode,
    ) -> Result<Self::UdpSocket, Self::Error> {
        let timeout_us = match mode {
            embedded_nal::Mode::Blocking => riot_sys::SOCK_NO_TIMEOUT,
            embedded_nal::Mode::NonBlocking => 0,
            embedded_nal::Mode::Timeout(millis) => (millis as u32) * 1000,
        };

        let local = match remote {
            embedded_nal::SocketAddr::V4(_) => riot_sys::init_SOCK_IPV4_EP_ANY(),
            embedded_nal::SocketAddr::V6(_) => riot_sys::init_SOCK_IPV6_EP_ANY(),
        };

        let mut socket = MaybeUninit::uninit();

        let remote: crate::socket::UdpEp = remote.into();
        let remote = remote.into();

        (unsafe { riot_sys::sock_udp_create(
                    socket.as_mut_ptr(),
                    &local as *const _ as *const _, // INLINE CAST
                    &remote,
                    0) })
            .negative_to_error()?;

        let socket = unsafe { socket.assume_init() };

        Ok(UdpSocket {
            socket,
            timeout_us
        })
    }
    fn write(
        &self,
        socket: &mut Self::UdpSocket,
        buffer: &[u8],
    ) -> Result<(), nb::Error<Self::Error>> {

        (unsafe { riot_sys::sock_udp_send(
                    &mut socket.socket,
                    buffer.as_ptr() as _,
                    buffer.len(),
                    0 as *const _,
                    ) })
            .negative_to_error()
            .map(|_| ())
            // Sending never blocks in RIOT sockets
            .map_err(|e| nb::Error::Other(e))
    }
    fn read(
        &self,
        socket: &mut Self::UdpSocket,
        buffer: &mut [u8],
    ) -> Result<usize, nb::Error<Self::Error>> {
        (unsafe { riot_sys::sock_udp_recv(
                    &mut socket.socket,
                    buffer.as_mut_ptr() as _,
                    buffer.len(),
                    socket.timeout_us,
                    0 as *mut _,
                    ) })
            .negative_to_error()
            .map(|e| e as usize)
            .map_err(|e| e.again_is_wouldblock())
    }

    fn close(&self, socket: Self::UdpSocket) -> Result<(), Self::Error> {
        unsafe { riot_sys::sock_udp_close(&mut *socket.socket) };
        Ok(())
    }
}
