use raw::{
    gnrc_netif_iter,
    gnrc_netif_t,
};

use ::core::iter::Iterator;

struct NetifIter {
    current: *const gnrc_netif_t,
}

impl Iterator for NetifIter {
    type Item = *const gnrc_netif_t;

    fn next(&mut self) -> Option<Self::Item>
    {
        self.current = unsafe { gnrc_netif_iter(self.current) };
        if self.current == 0 as *const gnrc_netif_t {
            None
        } else {
            Some(self.current)
        }
    }
}

pub fn netif_iter() -> impl Iterator<Item = *const gnrc_netif_t> {
    NetifIter { current: 0 as *const gnrc_netif_t }
}
