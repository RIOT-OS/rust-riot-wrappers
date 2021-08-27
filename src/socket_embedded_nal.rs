//! An implementation of the [embedded_nal] (Network Abstradtion Layer) UDP traits based on RIOT
//! sockets

use core::convert::TryInto;
use core::mem::MaybeUninit;

use crate::error::{NegativeErrorExt, NumericError};
use crate::socket::UdpEp;

use embedded_nal::SocketAddr;

/// The operating system's network stack, used to get an implementation of
/// ``embedded_nal::UdpClient``.
///
/// Using this is not trivial, as RIOT needs its sockets pinned to memory for their lifetime.
/// Without a heap allocator, this is achieved by allocating all the required UDP sockets in a
/// stack object. To ensure that it is not moved, sockets on it can only be created in (and live
/// only for the duration of) a the `run` callback, which gives the actual implemtation of
/// UdpClient.
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
        runner(accessor);
        // In particular, this would require tracking of whether the sockets are closed
        unimplemented!("Allocator does not have clean-up implemented");
    }
}

pub struct UdpSocket<'a> {
    socket: Option<&'a mut riot_sys::sock_udp_t>,
}

impl<'a> UdpSocket<'a> {
    /// Accessor to the inner socket pointer to mask the lack of type state in embedded-nal
    fn access(&mut self) -> Result<&mut riot_sys::sock_udp_t, NumericError> {
        self.socket
            .as_mut()
            .ok_or(NumericError::from_constant(riot_sys::ENOTCONN as _))
            .map(|s| &mut **s)
    }

    /// If there is an actuall socket in here, close it
    fn close(&mut self) {
        if let Some(socket) = self.socket.take() {
            unsafe { riot_sys::sock_udp_close(&mut *socket) };
        }
    }
}

impl<'a, const UDPCOUNT: usize> StackAccessor<'a, UDPCOUNT> {
    /// Take one of the stack accessor's allocated slots
    fn allocate(&self) -> Result<*mut riot_sys::sock_udp, NumericError>
    {
        let index = self.stack.udp_sockets_used.get();
        if index == UDPCOUNT {
            return Err(NumericError::from_constant(riot_sys::ENOMEM as _));
        }

        // We're inside a non-Send (due to the udp_sockets_used that's just cleared us as
        // the own allocation) function and thus will only ever be the single user of this slot.
        // Therefore, we can get ourselves a mutable pointer to the content of the UnsafeCell we
        // now own, and later make a mutable reference out of it
        let socket: *mut riot_sys::sock_udp_t = self.stack.udp_sockets[index].get() as *mut _;
        // FIXME replace bump allocator with anything more sensible that'd allow freeing
        self.stack.udp_sockets_used.set(index + 1);

        Ok(socket)
    }

    /// Wrapper around sock_udp_create
    fn create(
        &self,
        handle: &mut UdpSocket<'a>,
        local: &UdpEp,
        remote: Option<&UdpEp>,
    ) -> Result<(), NumericError> {
        handle.close();

        let socket = self.allocate()?;

        (unsafe {
            riot_sys::sock_udp_create(
                socket,
                local.as_ref(),
                remote.map(|r| {
                    let r: &riot_sys::sock_udp_ep_t = r.as_ref();
                    r as *const _})
                    .unwrap_or(core::ptr::null()),
                0,
            )
        })
        .negative_to_error()?;

        // unsafe: This is a manual assume_init (backed by the API), and having an 'a mutable
        // reference for it is OK because the StackAccessor guarantees that the stack is available
        // for 'a and won't move.
        let socket: &'a mut _ = unsafe { &mut *socket };

        handle.socket = Some(socket);

        Ok(())
    }
}

impl<'a, const UDPCOUNT: usize> embedded_nal::UdpClient for StackAccessor<'a, UDPCOUNT> {
    type UdpSocket = UdpSocket<'a>;
    type Error = NumericError;

    fn socket(&self) -> Result<UdpSocket<'a>, Self::Error> {
        Ok(UdpSocket { socket: None })
    }

    fn connect(
        &self,
        handle: &mut Self::UdpSocket,
        remote: SocketAddr,
    ) -> Result<(), Self::Error> {
        let local = match remote {
            SocketAddr::V4(_) => riot_sys::init_SOCK_IPV4_EP_ANY(),
            SocketAddr::V6(_) => riot_sys::init_SOCK_IPV6_EP_ANY(),
        }.into();

        let remote = remote.into();

        self.create(handle, &local, Some(&remote))
    }
    fn send(
        &self,
        socket: &mut Self::UdpSocket,
        buffer: &[u8],
    ) -> Result<(), nb::Error<Self::Error>> {
        let socket = socket.access()?;

        (unsafe {
            riot_sys::sock_udp_send(
                crate::inline_cast_mut(&mut *socket as *mut _),
                buffer.as_ptr() as _,
                buffer.len().try_into().unwrap(),
                0 as *const _,
            )
        })
        .negative_to_error()
        .map(|_| ())
        // Sending never blocks in RIOT sockets
        .map_err(|e| nb::Error::Other(e))
    }
    fn receive(
        &self,
        socket: &mut Self::UdpSocket,
        buffer: &mut [u8],
    ) -> Result<(usize, SocketAddr), nb::Error<Self::Error>> {
        let socket = socket.access()?;

        let mut remote = MaybeUninit::uninit();

        let read = (unsafe {
            riot_sys::sock_udp_recv(
                crate::inline_cast_mut(&mut *socket as *mut _),
                buffer.as_mut_ptr() as _,
                buffer.len().try_into().unwrap(),
                0,
                crate::inline_cast_mut(remote.as_mut_ptr() as *mut _),
            )
        })
            .negative_to_error()
            .map(|e| e as usize)
            .map_err(|e| e.again_is_wouldblock());

        // unsafe: Set by C function
        let remote = UdpEp(unsafe { remote.assume_init() });

        Ok((read?, remote.into()))
    }

    fn close(&self, mut socket: Self::UdpSocket) -> Result<(), Self::Error> {
        socket.close();
        Ok(())
    }
}

impl<'a, const UDPCOUNT: usize> embedded_nal::UdpServer for StackAccessor<'a, UDPCOUNT> {
    fn bind(&self, handle: &mut UdpSocket<'a>, port: u16) -> Result<(), Self::Error> {
        let local = UdpEp::ipv6_any()
            .with_port(port);

        self.create(handle, &local, None)
    }

    fn send_to(&self, handle: &mut UdpSocket<'a>, remote: SocketAddr, buffer: &[u8]) -> Result<(), nb::Error<Self::Error>> {
        let socket = handle.access()?;

        let remote: UdpEp = remote.into();

        (unsafe {
            riot_sys::sock_udp_send(
                crate::inline_cast_mut(&mut *socket as *mut _),
                buffer.as_ptr() as _,
                buffer.len().try_into().unwrap(),
                remote.as_ref(),
            )
        })
        .negative_to_error()
        .map(|_| ())
        // Sending never blocks in RIOT sockets
        .map_err(|e| nb::Error::Other(e))
    }
}
