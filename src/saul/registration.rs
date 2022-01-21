//! Tools for registering a Rust device in SAUL, see parent module documentation for details

use cstr_core::CStr;
use riot_sys::libc;

use super::{Class, Phydat};
use crate::error::NegativeErrorExt;
use crate::Never;

/// The single error read and write operations may produce; corresponds to an `-ECANCELED`.
/// (-ENOTSUP is expressed by not having support for the operation in the first place, indicated by
/// the `HAS_{READ,WRITE}` consts).
pub struct Error;

// Sync is required because callers from any thread may use the raw methods to construct a self
// reference through which it is used
pub trait Drivable: Sized + Sync + 'static {
    /// Sensor class (type)
    const CLASS: Class;

    /// Set to true if `read` is implemented.
    ///
    /// Doing this on the type level (rather than having read and write return a more
    /// differentiated error) allows the driver to point to the shared [riot_sys::saul_notsup]
    /// handler rather than to monomorphize a custom erring handler for each device.
    const HAS_READ: bool = false;
    /// Set to true if `write` is implemented.
    const HAS_WRITE: bool = false;

    /// Read the current state
    fn read(&self) -> Result<Phydat, Error> {
        // This function's presence in generated code should already show that something is
        // configured badly; could consider making that a linker error (but riot-wrappers is not in
        // the habit of doing that).
        unimplemented!("Sensor reading not implemented; HAS_READ should not have been set.")
    }

    /// Set the state of an actuator, or reconfigure a sensor
    ///
    /// A &self is passed in on write because there could be concurrent access from multiple SAUL
    /// users. One option of handling this is to implement Drivable for Mutex<T>.
    ///
    /// Note that due to the way SAUL is structured, the drivable can not know the number of
    /// entries which the user intended to set. The Drivable trait always builds the Rust Phydat
    /// (which contains a length) with the maximum available length (some of which may contain
    /// uninitialized data, which is OK as i16 has no uninhabited values), and the writer needs to
    /// return how many of the entries it actually used.
    fn write(&self, data: &Phydat) -> Result<u8, Error> {
        // See also comment in read()
        unimplemented!("Sensor writing not implemented; HAS_READ should not have been set.")
    }

    unsafe extern "C" fn read_raw(dev: *const libc::c_void, res: *mut riot_sys::phydat_t) -> i32 {
        let device = &*(dev as *const Self);
        match device.read() {
            Ok(d) => {
                res.write(d.values);
                d.length.into()
            }
            // The only legal device error -- ENOTSUP would mean there's no handler at all
            Err(_) => -(riot_sys::ECANCELED as i32),
        }
    }

    unsafe extern "C" fn write_raw(dev: *const libc::c_void, data: *mut riot_sys::phydat_t) -> i32 {
        let device = &*(dev as *const Self);
        let data = *data;
        // PHYDAT_DIM: See write documentation
        let data = Phydat {
            values: data,
            length: riot_sys::PHYDAT_DIM as _,
        };
        match Self::write(device, &data) {
            Ok(n) => n as _,
            // The only legal device error -- ENOTSUP would mean there's no handler at all
            Err(_) => -(riot_sys::ECANCELED as i32),
        }
    }
}

/// A typed saul_driver_t, created from a Drivable's build_driver() static method, and used as
/// statically lived references registrations.
pub struct Driver<D: Drivable> {
    driver: riot_sys::saul_driver_t,
    _phantom: core::marker::PhantomData<D>,
}

impl<D: Drivable> Driver<D> {
    pub const fn new() -> Self {
        Driver {
            driver: riot_sys::saul_driver_t {
                read: if D::HAS_READ { Some(D::read_raw) } else { Some(riot_sys::saul_notsup) },
                write: if D::HAS_WRITE { Some(D::write_raw) } else { Some(riot_sys::saul_notsup) },
                type_: D::CLASS.to_c(),
            },
            _phantom: core::marker::PhantomData,
        }
    }
}

// unsafe: All the content we have in the inner struct is Send (being just plain functions), so is
// the whole
unsafe impl<D: Drivable> Send for Driver<D> {}

pub struct Registration<D: Drivable> {
    reg: riot_sys::saul_reg_t,
    _phantom: core::marker::PhantomData<&'static D>,
}

// unsafe: The registration is as Send as the pointer to the Drivable it contains (and Drivable
// requires Sync)
unsafe impl<'a, D: Drivable> Send for Registration<D> {}

impl<D: Drivable> Registration<D> {
    // Unlike in the old implementation, no attempt is made to build it short-lived and then
    // upgrade -- not for lifetime reasons, but because for `register_static` all is already
    // static anyway, and `build_with` can just as well be called with the components, claim their
    // lifetime first and then go through here.
    pub fn new(
        driver: &'static Driver<D>,
        device: &'static D,
        name: Option<&'static CStr>,
    ) -> Self {
        Registration {
            reg: riot_sys::saul_reg_t {
                next: 0 as _,
                dev: device as *const _ as *mut _,
                name: name.map(|n| n.as_ptr()).unwrap_or(0 as _),
                driver: &driver.driver as *const _,
            },
            _phantom: core::marker::PhantomData,
        }
    }

    /// Hook the registration in with the global SAUL list
    ///
    /// If you can not obtain a &'static, you may consider [`register_and_then()`].
    pub fn register_static(&'static mut self) {
        (unsafe { riot_sys::saul_reg_add(&mut self.reg) })
            .negative_to_error()
            .expect("Constructed registries are always valid");
    }
}

/// Hook the registration in with the global SAUL list
///
/// Compared to [`Registration::register_static()`], this is convenient for threads that run
/// forever and which just need a reference to move into an infinitely executing closure to get the
/// same guarantees as from a static reference.
// It would be nice to have a helper function that proves the infinite lifetime independently
// of the registration, but none such is known.
//
// If unwinding is ever added in RIOT, this will need a guard similar to the one in the
// `replace_with` crate.
pub fn register_and_then<D: Drivable>(
    driver: &Driver<D>,
    device: &D,
    name: Option<&CStr>,
    f: impl FnOnce() -> Never,
) -> ! {
    // Reborrow for 'static lifetime.
    //
    // This is safe because we never terminate, and thus keep these references forever.
    // (It could be done prettier, but see above on preferring to have a proper library for
    // this anyway).
    let (driver, device, name) = unsafe { core::mem::transmute((driver, device, name)) };

    let mut registration = Registration::<D>::new(driver, device, name);

    (unsafe { riot_sys::saul_reg_add(&mut registration.reg) })
        .negative_to_error()
        .expect("Constructed registries are always valid");
    f()
}
