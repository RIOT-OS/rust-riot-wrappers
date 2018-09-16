// So far, this implements only the registration of own sensors. That is much easier to implement
// than reading from sensors, for there we'd have to construct lifetimes for things whose memory
// location is managed in C.

use riot_sys as raw;
use riot_sys::libc;

/// A struct containing both a registration and its driver
struct ContainedRegistration<R> {
    registration: raw::saul_reg_t,
    driver: raw::saul_driver_t,
    read: R,
    // Could probably go into the type information instead, but as long as there's no pinning
    // available, I'm careful (maybe not even enough) to not tempt Rust to move the code around by
    // consuming self and returning an almost-but-not-quite identical object on the stack again.
    registered: bool,
}

unsafe extern "C" fn no_writes(_dev: *const libc::c_void, _res: *mut raw::phydat_t) -> i32 {
    -(raw::ENODEV as i32)
}

pub trait SimpleSensor {
    fn start(&mut self) -> Result<(), ()>;
}

pub fn create_simple_sensor<R>(name: &libc::CStr, type_: u8, read: R) -> impl SimpleSensor
where
    R: FnMut(&mut raw::phydat_t) -> i32,
{
    ContainedRegistration {
        driver: raw::saul_driver_t {
            read: None,
            write: Some(no_writes),
            type_: type_,
        },
        registration: raw::saul_reg_t {
            next: 0 as *mut raw::saul_reg_t,
            dev: 0 as *mut libc::c_void,
            name: name.as_ptr(),
            driver: 0 as *mut raw::saul_driver_t,
        },
        read: read,
        registered: false,
    }
}

impl<R> ContainedRegistration<R>
where
    R: FnMut(&mut raw::phydat_t) -> i32,
{
    // This could do much more in terms of hiding the internals, but phydat_t is sufficiently easy
    // to use right now.
    unsafe extern "C" fn run_read(dev: *const libc::c_void, res: *mut raw::phydat_t) -> i32 {
        let self_ = &mut *(dev as *mut ContainedRegistration<R>);
        let res = &mut *res;
        let f = &mut self_.read;
        f(res)
    }
}

impl<R> SimpleSensor for ContainedRegistration<R>
where
    R: FnMut(&mut raw::phydat_t) -> i32,
{
    /// Do everything that could not be done in the constructor but needs the result to stay pinned
    /// -- even if it isn't officially expressed through any Pin marker, this hopefully doesn't
    /// move any more.
    fn start(&mut self) -> Result<(), ()> {
        self.registration.driver = &self.driver;
        self.registration.dev = self as *mut Self as *mut libc::c_void;
        self.driver.read = Some(ContainedRegistration::<R>::run_read);

        let success = unsafe { raw::saul_reg_add(&mut self.registration) };
        match success {
            0 => {
                self.registered = true;
                Ok(())
            }
            _ => Err(()),
        }
    }
}

impl<R> Drop for ContainedRegistration<R> {
    // should maybe already trigger on moves, and re-register after the moves?
    fn drop(&mut self) {
        if self.registered {
            // Not evaluating the error code, for what could we do...
            unsafe { raw::saul_reg_rm(&mut self.registration) };
        }
    }
}
