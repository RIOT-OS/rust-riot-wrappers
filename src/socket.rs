/// Wrappers for elements of the Sock API

pub struct UdpEp(riot_sys::sock_udp_ep_t);

impl Into<riot_sys::sock_udp_ep_t> for UdpEp {
    fn into(self) -> riot_sys::sock_udp_ep_t {
        self.0
    }
}

impl Into<UdpEp> for embedded_nal::SocketAddr {
    fn into(self) -> UdpEp {
        use embedded_nal::SocketAddr::*;

        UdpEp(
            riot_sys::sock_udp_ep_t {
                family: match self {
                    V4(_) => riot_sys::AF_INET as _,
                    V6(_) => riot_sys::AF_INET6 as _,
                    },
                addr: match self {
                    V4(_) => unimplemented!(),
                    V6(a) => riot_sys::_sock_tl_ep__bindgen_ty_1 {
                        ipv6: a.ip().octets()
                    },
                },
                netif: match self {
                    V4(_) => 0,
                    V6(a) => a.scope_id() as _,
                },
                port: self.port(),
            }
        )
    }
}
