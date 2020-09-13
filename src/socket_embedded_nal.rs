//! An implementation of the [embedded-nal] (Network Abstradtion Layer) UDP trait based on RIOT
//! sockets
//!
//! [embedded-nal]: https://docs.rs/embedded-nal/0.1.0/embedded_nal/

use core::mem::MaybeUninit;

use crate::error::NegativeErrorExt;

/// The operating system's network stack, used to get an implementation of ``embedded_nal::UdpStack``.
///
/// Using this is not trivial, as RIOT needs its sockets pinned to memory for their lifetime.
/// Without a heap allocator, this is achieved by allocating all the required UDP sockets in a
/// stack object. To ensure that it is not moved, sockets on it can only be created in (and live
/// only for the duration of) a the `run` callback, which gives the actual implemtation of
/// UdpStack.
///
/// The number of UDP sockets allocated is configurable using the UDPCOUNT const generic.
pub struct Stack<const UDPCOUNT: usize> {
    udp_sockets: [core::cell::UnsafeCell<MaybeUninit<riot_sys::sock_udp_t>>; UDPCOUNT],
    udp_sockets_used: core::cell::Cell<usize>,
}

pub struct StackAccessor<'a, const UDPCOUNT: usize> {
    stack: &'a Stack<UDPCOUNT>,
}

impl<const UDPCOUNT: usize> Stack<UDPCOUNT> {
    pub fn new() -> Self {
        Self {
            // FIXME this should work with #![feature(const_in_array_repeat_expressions)]
            // udp_sockets: [core::cell::UnsafeCell::new(MaybeUninit::uninit()); UDPCOUNT],
            // unsafe: OK because neither the array nor UnsafeCell nor MaybeUninit have any
            // uninhabited states
            udp_sockets: unsafe { MaybeUninit::uninit().assume_init() },
            udp_sockets_used: 0.into(),
        }
    }

    pub fn run(&self, runner: impl for<'a> FnOnce(StackAccessor<'a, UDPCOUNT>)) {
        let accessor = StackAccessor { stack: self };
        runner(accessor)
    }
}

pub struct UdpSocket<'a> {
    socket: &'a mut riot_sys::sock_udp_t,
    timeout_us: u32,
}

impl<'a, const UDPCOUNT: usize> embedded_nal::UdpStack for StackAccessor<'a, UDPCOUNT> {
    type UdpSocket = UdpSocket<'a>;
    type Error = crate::error::NumericError;

    fn open(
        &self,
        remote: embedded_nal::SocketAddr,
        mode: embedded_nal::Mode,
    ) -> Result<Self::UdpSocket, Self::Error> {
        let index = self.stack.udp_sockets_used.get();
        if index == UDPCOUNT {
            return Err(crate::error::NumericError::from(riot_sys::ENOMEM as _));
        }

        let socket: &'a mut riot_sys::sock_udp_t = unsafe {
            (&mut *self.stack.udp_sockets[index].get()).get_mut()
        };
        // FIXME replace bump allocator with anything more sensible that'd allow freeing
        self.stack.udp_sockets_used.set(index + 1);

        let timeout_us = match mode {
            embedded_nal::Mode::Blocking => riot_sys::SOCK_NO_TIMEOUT,
            embedded_nal::Mode::NonBlocking => 0,
            embedded_nal::Mode::Timeout(millis) => (millis as u32) * 1000,
        };

        let local = match remote {
            embedded_nal::SocketAddr::V4(_) => riot_sys::init_SOCK_IPV4_EP_ANY(),
            embedded_nal::SocketAddr::V6(_) => riot_sys::init_SOCK_IPV6_EP_ANY(),
        };

        let remote: crate::socket::UdpEp = remote.into();
        let remote = remote.into();

        (unsafe { riot_sys::sock_udp_create(
                    socket,
                    &local as *const _ as *const _, // INLINE CAST
                    &remote,
                    0) })
            .negative_to_error()?;

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
                    &mut *socket.socket,
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
                    &mut *socket.socket,
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
