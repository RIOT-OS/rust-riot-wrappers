use riot_sys::{
    gnrc_nettype_t,
    gnrc_netreg_entry_t,
    gnrc_netreg_register,
    gnrc_netreg_unregister,
};

pub struct Registration<'a> {
    nettype: gnrc_nettype_t,
    entry: &'a mut gnrc_netreg_entry_t,
}

impl<'a> Registration<'a> {
    pub fn new(nettype: gnrc_nettype_t, entry: &'a mut gnrc_netreg_entry_t) -> Self {
        let result = unsafe { gnrc_netreg_register(nettype, entry) };
        assert!(result == 0);
        Self { nettype, entry }
    }
}

impl<'a> Drop for Registration<'a> {
    fn drop(&mut self) {
        unsafe { gnrc_netreg_unregister(self.nettype, self.entry) };
    }
}
