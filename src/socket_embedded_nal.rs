//! An implementation of the [embedded_nal] (Network Abstradtion Layer) UDP traits based on RIOT
//! sockets

use core::convert::TryInto;
use core::mem::MaybeUninit;

use crate::error::{NegativeErrorExt, NumericError};

use embedded_nal::SocketAddr;

fn ep_to_sockaddr(ep: &riot_sys::sock_udp_ep_t) -> SocketAddr {
    // FIXME deduplicate with UdpEp (not trivial because through pointer)
    match ep.family as _ {
        riot_sys::AF_INET6 =>
            embedded_nal::SocketAddrV6::new(
                    // unsafe: Access to typed C union
                    unsafe { ep.addr.ipv6.into() },
                    ep.port,
                    0,
                    ep.netif.into(),
                ).into(),

        riot_sys::AF_INET =>
            embedded_nal::SocketAddrV4::new(
                    // unsafe: Access to typed C union
                    unsafe { ep.addr.ipv4.into() },
                    ep.port
                ).into(),

        _ => panic!("Endpoint not expressible in embedded_nal"),
    }
}

fn sockaddr_to_ep(sa: &SocketAddr) -> riot_sys::sock_udp_ep_t {
    // FIXME algin with UdpEp
    match sa {
        SocketAddr::V6(sa) => todo!(),
        SocketAddr::V4(sa) => {
            let mut ep: riot_sys::sock_udp_ep_t = Default::default();
            ep.family = riot_sys::AF_INET as _;
            ep.addr.ipv4 = sa.ip().octets();
            ep.port = sa.port();
            ep
        },
    }
}

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
        runner(accessor)
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
        local: &riot_sys::inline::sock_udp_ep_t,
        remote: &riot_sys::sock_udp_ep_t,
    ) -> Result<(), NumericError> {
        handle.close();

        let socket = self.allocate()?;

        (unsafe {
            riot_sys::sock_udp_create(
                socket,
                local as *const _ as *const _, // INLINE CAST
                remote as *const _ as *const _, // INLINE CAST
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
        };

        let remote: crate::socket::UdpEp = remote.into();
        let remote: riot_sys::sock_udp_ep_t = remote.into();

        self.create(handle, &local, &remote)
    }
    fn send(
        &self,
        socket: &mut Self::UdpSocket,
        buffer: &[u8],
    ) -> Result<(), nb::Error<Self::Error>> {
        let socket = socket.access()?;

        (unsafe {
            riot_sys::sock_udp_send(
                &mut *socket as *mut _ as *mut _, // INLINE CAST
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
                &mut *socket as *mut _ as *mut _, // INLINE CAST
                buffer.as_mut_ptr() as _,
                buffer.len().try_into().unwrap(),
                0,
                remote.as_mut_ptr() as *mut _ as *mut _,
            )
        })
            .negative_to_error()
            .map(|e| e as usize)
            .map_err(|e| e.again_is_wouldblock());

        // unsafe: Set by C function
        let remote = unsafe { remote.assume_init() };

        Ok((read?, ep_to_sockaddr(&remote)))
    }

    fn close(&self, mut socket: Self::UdpSocket) -> Result<(), Self::Error> {
        socket.close();
        Ok(())
    }
}

impl<'a, const UDPCOUNT: usize> embedded_nal::UdpServer for StackAccessor<'a, UDPCOUNT> {
    fn bind(&self, handle: &mut UdpSocket<'a>, port: u16) -> Result<(), Self::Error> {
        let mut local = riot_sys::init_SOCK_IPV6_EP_ANY();
        local.port = port;
        let remote = riot_sys::init_SOCK_IPV6_EP_ANY();

        self.create(handle, &local, unsafe { core::mem::transmute(&remote) } ) // FIXME INLINE CAST
    }

    fn send_to(&self, handle: &mut UdpSocket<'a>, remote: SocketAddr, buffer: &[u8]) -> Result<(), nb::Error<Self::Error>> {
        let socket = handle.access()?;

        let remote = sockaddr_to_ep(&remote);

        (unsafe {
            riot_sys::sock_udp_send(
                &mut *socket as *mut _ as *mut _, // INLINE CAST
                buffer.as_ptr() as _,
                buffer.len().try_into().unwrap(),
                &remote as *const _ as *const _, // INLINE CAST
            )
        })
        .negative_to_error()
        .map(|_| ())
        // Sending never blocks in RIOT sockets
        .map_err(|e| nb::Error::Other(e))
    }
}
