use crate::error::NegativeErrorExt;
use core::mem::MaybeUninit;

#[derive(Debug)]
pub struct UdpStack {
    // It's annoying we need those not to move; just asking the user to help us out here.
    static_socket_factory: fn() -> Option<&'static mut MaybeUninit<riot_sys::sock_udp_t>>,
}

impl UdpStack {
    pub fn new(factory: fn() -> Option<&'static mut MaybeUninit<riot_sys::sock_udp_t>>) -> Self {
        Self {
            static_socket_factory: factory,
        }
    }
}

impl embedded_nal_async::UdpStack for UdpStack {
    type Error = crate::error::NumericError;
    type Connected = ConnectedUdpSocket;
    type UniquelyBound = UnconnectedUdpSocket;
    type MultiplyBound = UnconnectedUdpSocket;

    async fn connect_from(
        &self,
        local: embedded_nal_async::SocketAddr,
        remote: embedded_nal_async::SocketAddr,
    ) -> Result<(embedded_nal_async::SocketAddr, Self::Connected), Self::Error> {
        let local: crate::socket::UdpEp = local.into();
        let remote: crate::socket::UdpEp = remote.into();
        let mut socket = (self.static_socket_factory)().ok_or(
            crate::error::NumericError::from_constant(riot_sys::ENOSPC as _),
        )?;
        let mut socket = unsafe {
            riot_sys::sock_udp_create(socket.as_mut_ptr(), local.as_ref(), remote.as_ref(), 0)
                .negative_to_error()?;
            socket.assume_init_mut()
        };

        let mut final_local = MaybeUninit::uninit();
        let final_local: crate::socket::UdpEp = unsafe {
            riot_sys::sock_udp_get_local(socket, final_local.as_mut_ptr()).negative_to_error()?;
            final_local.assume_init()
        }
        .into();

        // FIXME: Verify that sock really narrows local address in case something unspecified was
        // passed in
        // (may need additional sock features https://matrix.to/#/!pqHdpanAvkJvlCwUDE:matrix.org/$njHr_rLw0yWUceYytDYgsop1Aiz9p0WSTp4HN6NBtPA?via=matrix.org&via=utwente.io&via=rubdos.be )
        // (first tests indicate that while the UDP port is fixed, the local address is not)
        crate::println!(
            "Afer connection, local address went from {:?} to {:?}",
            local,
            final_local
        );

        Ok((final_local.into(), ConnectedUdpSocket { socket }))
    }

    async fn bind_single(
        &self,
        local: embedded_nal_async::SocketAddr,
    ) -> Result<(embedded_nal_async::SocketAddr, Self::UniquelyBound), Self::Error> {
        todo!()
    }
    async fn bind_multiple(
        &self,
        local: embedded_nal_async::SocketAddr,
    ) -> Result<Self::MultiplyBound, Self::Error> {
        todo!()
    }
}

impl embedded_io_async::Error for crate::error::NumericError {
    fn kind(&self) -> embedded_io_async::ErrorKind {
        // FIXME there are some that do make sense here
        embedded_io_async::ErrorKind::Other
    }
}

#[derive(Debug)]
pub struct ConnectedUdpSocket {
    socket: &'static mut riot_sys::sock_udp_t,
}

impl embedded_nal_async::ConnectedUdp for ConnectedUdpSocket {
    type Error = crate::error::NumericError;

    async fn send(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        unsafe {
            riot_sys::sock_udp_send(
                crate::inline_cast_ref_mut(self.socket),
                data.as_ptr() as _,
                data.len() as _,
                core::ptr::null(),
            )
            .negative_to_error()?
        };
        Ok(())
    }
    async fn receive_into(&mut self, buffer: &mut [u8]) -> Result<usize, Self::Error> {
        use crate::async_helpers::{RiotStyleFuture, RiotStylePollStruct};

        struct ReceiveIntoArgs<'a> {
            sock: &'a mut riot_sys::sock_udp_t,
            buffer: &'a mut [u8],
        }
        impl RiotStyleFuture for ReceiveIntoArgs<'_> {
            type Output = Result<usize, crate::error::NumericError>;
            fn poll(
                &mut self,
                arg: *mut riot_sys::libc::c_void,
            ) -> core::task::Poll<Result<usize, crate::error::NumericError>> {
                let sock: &mut riot_sys::sock_udp_t = self.sock;
                // FIXME: This is not complying with the embedded-nal-async API, because it errs on
                // overly long received packets, rather than returning the total length while
                // filling up the buffer. (But let's clean that up once there is less other
                // refactoring to do around here).
                let result = (unsafe {
                    riot_sys::sock_udp_recv(
                        crate::inline_cast_mut(sock),
                        self.buffer.as_ptr() as _,
                        self.buffer.len() as _,
                        // Return immediately
                        0,
                        // Not interested: This is a connected socket, it better be coming from the
                        // address we connected to.
                        core::ptr::null_mut(),
                    )
                })
                .negative_to_error();
                if result
                    == Err(crate::error::NumericError::from_constant(
                        riot_sys::EAGAIN as _,
                    ))
                {
                    // When setting all this up, we got a &mut self, so we can be sure that there
                    // is no other thread or task simultaneously trying to receive on this, or even
                    // doing a blocking send on it (which we currently don't do anyway).
                    //
                    // (Otherwise, we'd have to worry about overwriting a callback).
                    unsafe {
                        riot_sys::sock_udp_set_cb(sock, Some(Self::callback), arg);
                    }
                    core::task::Poll::Pending
                } else {
                    core::task::Poll::Ready(result.map(|n| n as _))
                }
            }
        }
        impl ReceiveIntoArgs<'_> {
            unsafe extern "C" fn callback(
                sock: *mut riot_sys::sock_udp_t,
                flags: riot_sys::sock_async_flags_t,
                arg: *mut riot_sys::libc::c_void,
            ) {
                if flags & riot_sys::inline::SOCK_ASYNC_MSG_RECV == 0 {
                    // We can get stray _MSG_SENT in here (although I'm not sure those even get
                    // generated currently). Waking once would do no harm, as the polling function
                    // would just find that the socket is surprisingly not ready and register the
                    // callback anew, but let's not kick that off if two instructions can prevent
                    // it.
                    return;
                }
                RiotStylePollStruct::<Self>::callback(arg);
            }
        }
        impl Drop for ReceiveIntoArgs<'_> {
            fn drop(&mut self) {
                let sock: &mut riot_sys::sock_udp_t = self.sock;
                unsafe {
                    riot_sys::sock_udp_set_cb(sock, None, core::ptr::null_mut());
                }
                // No need to drop our fields individually, they're all just references. But if we
                // had any, we'd need to drop them only after we know that no callback will fire
                // any more.
            }
        }

        RiotStylePollStruct::new(ReceiveIntoArgs {
            sock: self.socket,
            buffer,
        })
        .await
    }
}

pub struct UnconnectedUdpSocket {
    socket: riot_sys::sock_udp_t,
}

impl embedded_nal_async::UnconnectedUdp for UnconnectedUdpSocket {
    type Error = crate::error::NumericError;

    async fn send(
        &mut self,
        local: embedded_nal_async::SocketAddr,
        remote: embedded_nal_async::SocketAddr,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        todo!()
    }
    async fn receive_into(
        &mut self,
        buffer: &mut [u8],
    ) -> Result<
        (
            usize,
            embedded_nal_async::SocketAddr,
            embedded_nal_async::SocketAddr,
        ),
        Self::Error,
    > {
        todo!()
    }
}
