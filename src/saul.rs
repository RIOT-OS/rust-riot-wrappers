type saul_reg_t = usize; // lying. we just don't intend to dereference it, ever.
#[derive(Debug)]
#[repr(C)]
pub struct phydat_t {
    pub val: [i16; 3],
    pub unit: u8,
    pub scale: i8,
}

extern "C" {
    // Using raw pointers and not &'static because we saul lifetimes are not managed in a way
    // compatible with Rust -- in theory, a sensor could deregister itself in an interrupt.
    pub fn saul_reg_read(dev: *const saul_reg_t, res: &mut phydat_t) -> isize;
    pub fn saul_reg_find_nth(num: isize) -> *const saul_reg_t;
}
