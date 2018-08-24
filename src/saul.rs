
// extern "C" {
//     // Using raw pointers and not &'static because we saul lifetimes are not managed in a way
//     // compatible with Rust -- in theory, a sensor could deregister itself in an interrupt.
//     pub fn saul_reg_read(dev: *const saul_reg_t, res: &mut phydat_t) -> isize;
//     pub fn saul_reg_find_nth(num: isize) -> *const saul_reg_t;
// }
