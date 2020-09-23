/// Registration and use of SAUL, the Sensor Actuator Uber Layer

use core::convert::TryFrom;

use riot_sys as raw;
use riot_sys::libc;

use crate::error;
use error::NegativeErrorExt;

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


/// A discovered SAUL registry entry
pub struct RegistryEntry(*mut riot_sys::saul_reg);

impl RegistryEntry {
    /// Find a registry entry by its index
    ///
    /// Wrapper around `saul_reg_find_nth`.
    pub fn nth(pos: isize) -> Option<Self> {
        // unsafe: all positions are valid, and if it's not null, it's a static pointer as SAUL
        // registrations can't really be removed.
        (unsafe {
            riot_sys::saul_reg_find_nth(pos as _)
                .as_mut()
        }).map(|r| RegistryEntry(r))
    }

    pub fn all() -> impl Iterator<Item=Self> {
        // Could alternatively be implemented by hopping through the list's next pointer -- more
        // efficient, but relies on internals that are not part of the API
        //
        // (The shell command in sc_saul_reg also jumps through next, but that's in-tree.)
        (0..)
            .map(|n| Self::nth(n))
            .map_while(|p| p)
    }

    pub fn type_(&self) -> Option<Class> {
        // unsafe: Registrations are stable
        let type_ = unsafe { (*(*self.0).driver).type_ };
        Class::from_c(type_)
    }

    pub fn name(&self) -> Option<&'static str> {
        // unsafe: Registrations are stable, and point to null-terminated strings or are NULL.
        unsafe { Some((*self.0).name
            .as_ref()
            .map(|s| {
                riot_sys::libc::CStr::from_ptr(s as _)
                    .to_str()
                    .ok()
            })??)
        }
    }

    /// Read a value from the SAUL device
    pub fn read(&self) -> Phydat {
        // Could work with MaybeUninit here, but probably not worth it.
        let mut result: Phydat = Default::default();
        let length = unsafe { riot_sys::saul_reg_read(self.0 as *mut _, &mut result.values) };
        result.length = length as _;
        result
    }

    /// Write a value to the SAUL device
    pub fn write(&self, value: Phydat) -> Result<(), error::NumericError> {
        // Value copied as we can't really be sure that no SAUL device will ever write here
        unsafe { riot_sys::saul_reg_write(self.0, &value as *const _ as *mut _) }.negative_to_error()?;
        Ok(())
    }
}

/// A wrapper around phydat_t that keeps the values and the number of valid values in one place.
#[derive(Default, Copy, Clone)]
pub struct Phydat {
    values: riot_sys::phydat_t,
    // Always <= PHYDAT_DIM
    length: u8,
}

impl Phydat {
    /// Create a new phydat data value.
    ///
    /// # Panics
    ///
    /// This function panics if data length exceeds PHYDAT_DIM (3).
    pub fn new(data: &[i16], unit: Option<Unit>, scale: i8) -> Self {
        // Working with MaybeUninit might be legal here but is probably not worth it.
        let mut val = [0; riot_sys::PHYDAT_DIM as _];
        val[..data.len()].copy_from_slice(data);
        Phydat {
            values: riot_sys::phydat_t {
                val,
                unit: Unit::to_c(unit),
                scale: scale
            },
            length: data.len() as _,
        }
    }
}

impl core::fmt::Debug for Phydat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(
            f,
            "Phydat {{ {:?} x 10^{} {:?} }}",
            &self.values.val[..self.length as _],
            self.values.scale,
            Unit::from_c(self.values.unit)
            )
    }
}

/// Device class
///
/// Both for the class in general and for its details, Option is used to represent undefined /
/// unknown values, which are used as a wildcard in queries and as an indicator of unknown /
/// unspported types in introspection.
#[derive(Copy, Clone, Debug)]
pub enum Class {
    Actuator(Option<ActuatorClass>),
    Sensor(Option<SensorClass>)
}

impl Class {
    fn from_c(input: u8) -> Option<Class> {
        use Class::*;
        use ActuatorClass::*;
        use SensorClass::*;

        match input as _ {
            riot_sys::SAUL_ACT_ANY => Some(Actuator(None)),
            riot_sys::SAUL_ACT_LED_RGB => Some(Actuator(Some(LedRgb))),
            riot_sys::SAUL_ACT_SERVO => Some(Actuator(Some(Servo))),
            riot_sys::SAUL_ACT_MOTOR => Some(Actuator(Some(Motor))),
            riot_sys::SAUL_ACT_SWITCH => Some(Actuator(Some(Switch))),
            riot_sys::SAUL_ACT_DIMMER => Some(Actuator(Some(Dimmer))),
            riot_sys::SAUL_SENSE_ANY => Some(Sensor(None)),
            riot_sys::SAUL_SENSE_BTN => Some(Sensor(Some(Btn))),
            riot_sys::SAUL_SENSE_TEMP => Some(Sensor(Some(Temp))),
            riot_sys::SAUL_SENSE_HUM => Some(Sensor(Some(Hum))),
            riot_sys::SAUL_SENSE_LIGHT => Some(Sensor(Some(Light))),
            riot_sys::SAUL_SENSE_ACCEL => Some(Sensor(Some(Accel))),
            riot_sys::SAUL_SENSE_MAG => Some(Sensor(Some(Mag))),
            riot_sys::SAUL_SENSE_GYRO => Some(Sensor(Some(Gyro))),
            riot_sys::SAUL_SENSE_COLOR => Some(Sensor(Some(Color))),
            riot_sys::SAUL_SENSE_PRESS => Some(Sensor(Some(Press))),
            riot_sys::SAUL_SENSE_ANALOG => Some(Sensor(Some(Analog))),
            riot_sys::SAUL_SENSE_UV => Some(Sensor(Some(Uv))),
            riot_sys::SAUL_SENSE_OBJTEMP => Some(Sensor(Some(Objtemp))),
            riot_sys::SAUL_SENSE_COUNT => Some(Sensor(Some(Count))),
            riot_sys::SAUL_SENSE_DISTANCE => Some(Sensor(Some(Distance))),
            riot_sys::SAUL_SENSE_CO2 => Some(Sensor(Some(Co2))),
            riot_sys::SAUL_SENSE_TVOC => Some(Sensor(Some(Tvoc))),
            riot_sys::SAUL_SENSE_GAS => Some(Sensor(Some(Gas))),
            riot_sys::SAUL_SENSE_OCCUP => Some(Sensor(Some(Occup))),
            riot_sys::SAUL_SENSE_PROXIMITY => Some(Sensor(Some(Proximity))),
            riot_sys::SAUL_SENSE_RSSI => Some(Sensor(Some(Rssi))),
            riot_sys::SAUL_SENSE_CHARGE => Some(Sensor(Some(Charge))),
            riot_sys::SAUL_SENSE_CURRENT => Some(Sensor(Some(Current))),
            riot_sys::SAUL_SENSE_PM => Some(Sensor(Some(Pm))),
            riot_sys::SAUL_SENSE_CAPACITANCE => Some(Sensor(Some(Capacitance))),
            riot_sys::SAUL_SENSE_VOLTAGE => Some(Sensor(Some(Voltage))),
            riot_sys::SAUL_SENSE_PH => Some(Sensor(Some(Ph))),
            riot_sys::SAUL_SENSE_POWER => Some(Sensor(Some(Power))),
            riot_sys::SAUL_SENSE_SIZE => Some(Sensor(Some(Size))),
            x if x & riot_sys::SAUL_CAT_MASK == riot_sys::SAUL_CAT_ACT => Some(Actuator(None)),
            x if x & riot_sys::SAUL_CAT_MASK == riot_sys::SAUL_CAT_SENSE => Some(Sensor(None)),
            _ => None
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum ActuatorClass {
    LedRgb,
    Servo,
    Motor,
    Switch,
    Dimmer,
}

#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum SensorClass {
    Btn,
    Temp,
    Hum,
    Light,
    Accel,
    Mag,
    Gyro,
    Color,
    Press,
    Analog,
    Uv,
    Objtemp,
    Count,
    Distance,
    Co2,
    Tvoc,
    Gas,
    Occup,
    Proximity,
    Rssi,
    Charge,
    Current,
    Pm,
    Capacitance,
    Voltage,
    Ph,
    Power,
    Size,
}

#[derive(Copy, Clone, Debug)]
pub enum Unit {
    /// Note that this means "data has no physical unit", and is distinct from "No unit given",
    /// which is `Option::<Unit>::None` as opposed to `Some(Unit::None)`.
    None,
    TempC,
    TempF,
    TempK,
    Lux,
    M,
    M2,
    M3,
    G,
    Dps,
    Gr,
    A,
    V,
    W,
    Gs,
    Dbm,
    Coulomb,
    F,
    Ohm,
    Ph,
    Bar,
    Pa,
    Cd,
    Bool,
    Cts,
    Percent,
    Permill,
    Ppm,
    Ppb,
    Time,
    Date,
    Gpm3,
    Cpm3,
}

impl Unit {
    fn from_c(input: u8) -> Option<Self> {
        match input as _{
            riot_sys::UNIT_NONE => Some(Unit::None),
            riot_sys::UNIT_TEMP_C => Some(Unit::TempC),
            riot_sys::UNIT_TEMP_F => Some(Unit::TempF),
            riot_sys::UNIT_TEMP_K => Some(Unit::TempK),
            riot_sys::UNIT_LUX => Some(Unit::Lux),
            riot_sys::UNIT_M => Some(Unit::M),
            riot_sys::UNIT_M2 => Some(Unit::M2),
            riot_sys::UNIT_M3 => Some(Unit::M3),
            riot_sys::UNIT_G => Some(Unit::G),
            riot_sys::UNIT_DPS => Some(Unit::Dps),
            riot_sys::UNIT_GR => Some(Unit::Gr),
            riot_sys::UNIT_A => Some(Unit::A),
            riot_sys::UNIT_V => Some(Unit::V),
            riot_sys::UNIT_W => Some(Unit::W),
            riot_sys::UNIT_GS => Some(Unit::Gs),
            riot_sys::UNIT_DBM => Some(Unit::Dbm),
            riot_sys::UNIT_COULOMB => Some(Unit::Coulomb),
            riot_sys::UNIT_F => Some(Unit::F),
            riot_sys::UNIT_OHM => Some(Unit::Ohm),
            riot_sys::UNIT_PH => Some(Unit::Ph),
            riot_sys::UNIT_BAR => Some(Unit::Bar),
            riot_sys::UNIT_PA => Some(Unit::Pa),
            riot_sys::UNIT_CD => Some(Unit::Cd),
            riot_sys::UNIT_BOOL => Some(Unit::Bool),
            riot_sys::UNIT_CTS => Some(Unit::Cts),
            riot_sys::UNIT_PERCENT => Some(Unit::Percent),
            riot_sys::UNIT_PERMILL => Some(Unit::Permill),
            riot_sys::UNIT_PPM => Some(Unit::Ppm),
            riot_sys::UNIT_PPB => Some(Unit::Ppb),
            riot_sys::UNIT_TIME => Some(Unit::Time),
            riot_sys::UNIT_DATE => Some(Unit::Date),
            riot_sys::UNIT_GPM3 => Some(Unit::Gpm3),
            riot_sys::UNIT_CPM3 => Some(Unit::Cpm3),
            _ => None,
        }
    }

    fn to_c(input: Option<Self>) -> u8 {
        (match input {
            Some(Unit::None) => riot_sys::UNIT_NONE,
            Some(Unit::TempC) => riot_sys::UNIT_TEMP_C,
            Some(Unit::TempF) => riot_sys::UNIT_TEMP_F,
            Some(Unit::TempK) => riot_sys::UNIT_TEMP_K,
            Some(Unit::Lux) => riot_sys::UNIT_LUX,
            Some(Unit::M) => riot_sys::UNIT_M,
            Some(Unit::M2) => riot_sys::UNIT_M2,
            Some(Unit::M3) => riot_sys::UNIT_M3,
            Some(Unit::G) => riot_sys::UNIT_G,
            Some(Unit::Dps) => riot_sys::UNIT_DPS,
            Some(Unit::Gr) => riot_sys::UNIT_GR,
            Some(Unit::A) => riot_sys::UNIT_A,
            Some(Unit::V) => riot_sys::UNIT_V,
            Some(Unit::W) => riot_sys::UNIT_W,
            Some(Unit::Gs) => riot_sys::UNIT_GS,
            Some(Unit::Dbm) => riot_sys::UNIT_DBM,
            Some(Unit::Coulomb) => riot_sys::UNIT_COULOMB,
            Some(Unit::F) => riot_sys::UNIT_F,
            Some(Unit::Ohm) => riot_sys::UNIT_OHM,
            Some(Unit::Ph) => riot_sys::UNIT_PH,
            Some(Unit::Bar) => riot_sys::UNIT_BAR,
            Some(Unit::Pa) => riot_sys::UNIT_PA,
            Some(Unit::Cd) => riot_sys::UNIT_CD,
            Some(Unit::Bool) => riot_sys::UNIT_BOOL,
            Some(Unit::Cts) => riot_sys::UNIT_CTS,
            Some(Unit::Percent) => riot_sys::UNIT_PERCENT,
            Some(Unit::Permill) => riot_sys::UNIT_PERMILL,
            Some(Unit::Ppm) => riot_sys::UNIT_PPM,
            Some(Unit::Ppb) => riot_sys::UNIT_PPB,
            Some(Unit::Time) => riot_sys::UNIT_TIME,
            Some(Unit::Date) => riot_sys::UNIT_DATE,
            Some(Unit::Gpm3) => riot_sys::UNIT_GPM3,
            Some(Unit::Cpm3) => riot_sys::UNIT_CPM3,
            None => riot_sys::UNIT_UNDEF,
        }) as _
    }
}
