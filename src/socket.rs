/// Wrappers for elements of the Sock API

pub struct UdpEp(pub(crate) riot_sys::sock_udp_ep_t);

impl UdpEp {
    #[doc(alias = "SOCK_IPV6_EP_ANY")]
    pub fn ipv6_any() -> Self {
        riot_sys::init_SOCK_IPV6_EP_ANY().into()
    }

    #[doc(alias = "SOCK_IPV4_EP_ANY")]
    pub fn ipv4_any() -> Self {
        riot_sys::init_SOCK_IPV4_EP_ANY().into()
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.0.port = port;
        self
    }
}

impl From<riot_sys::sock_udp_ep_t> for UdpEp {
    fn from(ep: riot_sys::sock_udp_ep_t) -> Self {
        Self(ep)
    }
}

impl From<riot_sys::inline::sock_udp_ep_t> for UdpEp {
    fn from(ep: riot_sys::inline::sock_udp_ep_t) -> Self {
        // INLINE TRANSMUTE
        unsafe { Self(core::mem::transmute(ep)) }
    }
}

impl AsRef<riot_sys::sock_udp_ep_t> for UdpEp {
    fn as_ref(&self) -> &riot_sys::sock_udp_ep_t {
        &self.0
    }
}

impl AsMut<riot_sys::sock_udp_ep_t> for UdpEp {
    fn as_mut(&mut self) -> &mut riot_sys::sock_udp_ep_t {
        &mut self.0
    }
}

impl AsRef<riot_sys::inline::sock_udp_ep_t> for UdpEp {
    fn as_ref(&self) -> &riot_sys::inline::sock_udp_ep_t {
        unsafe { crate::inline_cast_ref(&self.0) }
    }
}

impl AsMut<riot_sys::inline::sock_udp_ep_t> for UdpEp {
    fn as_mut(&mut self) -> &mut riot_sys::inline::sock_udp_ep_t {
        unsafe { crate::inline_cast_ref_mut(&mut self.0) }
    }
}

impl Into<riot_sys::sock_udp_ep_t> for UdpEp {
    fn into(self) -> riot_sys::sock_udp_ep_t {
        self.0
    }
}

#[cfg(feature = "with_embedded_nal")]
mod nal_impls {
    use super::*;
    use embedded_nal::SocketAddr;

    impl Into<UdpEp> for &SocketAddr {
        fn into(self) -> UdpEp {
            use SocketAddr::*;

            // Constructing via default avoids using the volatile names of the union types
            let mut ep: riot_sys::sock_udp_ep_t = Default::default();

            ep.family =  match self {
                V4(_) => riot_sys::AF_INET as _,
                V6(_) => riot_sys::AF_INET6 as _,
            };
            ep.netif = match self {
                V4(_) => 0,
                V6(a) => a.scope_id() as _,
            };
            ep.port = self.port();
            match self {
                V4(a) => {
                    ep.addr.ipv4 = a.ip().octets();
                }
                V6(a) => {
                    ep.addr.ipv6 = a.ip().octets();
                }
            }

            UdpEp(ep)
        }
    }

    impl Into<UdpEp> for SocketAddr {
        fn into(self) -> UdpEp {
            (&self).into()
        }
    }

    impl Into<SocketAddr> for &UdpEp {
        fn into(self) -> SocketAddr {
            match self.0.family as _ {
                riot_sys::AF_INET6 =>
                    embedded_nal::SocketAddrV6::new(
                            // unsafe: Access to C union whose type was just checked
                            unsafe { self.0.addr.ipv6.into() },
                            self.0.port,
                            0,
                            self.0.netif.into(),
                        ).into(),

                riot_sys::AF_INET =>
                    embedded_nal::SocketAddrV4::new(
                            // unsafe: Access to C union whose type was just checked
                            unsafe { self.0.addr.ipv4.into() },
                            self.0.port
                        ).into(),

                _ => panic!("Endpoint not expressible in embedded_nal"),
            }
        }
    }

    impl Into<SocketAddr> for UdpEp {
        fn into(self) -> SocketAddr {
            (&self).into()
        }
    }
}
