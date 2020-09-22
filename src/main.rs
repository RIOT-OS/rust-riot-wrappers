use crate::stdio;
use core::fmt;
use core::fmt::Write;

/// To have a nice Rust main function, run the `riot_main!` macro with the name of your main
/// function an item (ie. top level in a module) in your crate. The function identified by it must
/// return something that implements the Termination trait.
///
/// Example:
///
/// ```
/// riot_main!(main);
///
/// fn main() {
///     unimplemented!()
/// }
/// ```
#[macro_export]
macro_rules! riot_main {
    ($main:ident) => {
        #[export_name = "main"]
        pub extern "C" fn c_main() -> u32 {
            use riot_wrappers::main::Termination;
            $main().report()
        }
    };
}

/// A result trait for main methods, analogous to std::process::Termination
pub trait Termination {
    fn report(self) -> u32;
}

impl Termination for () {
    fn report(self) -> u32 {
        0
    }
}

// Copied, stripped down from std and printlns replaced with riot-wrapper stdio

impl<E: fmt::Debug> Termination for Result<(), E> {
    fn report(self) -> u32 {
        match self {
            Ok(()) => ().report(),
            Err(err) => Err::<!, _>(err).report(),
        }
    }
}

impl Termination for ! {
    fn report(self) -> u32 {
        self
    }
}

impl<E: fmt::Debug> Termination for Result<!, E> {
    fn report(self) -> u32 {
        match self {
            Err(err) => {
                let mut stdout = stdio::Stdio {};
                writeln!(stdout, "Error: {:?}", err).unwrap();
                1
            }
            _ => unreachable!(),
        }
    }
}

// General alternative to this module: Build the extern "C" main all the time and request that the
// application implement a named function. I never got the main function to be carried to the
// linker step, though. If implemented like this, the module needs to be gated like
// set_panic_handler.

// extern "Rust" {
//     fn riot_main();
// }
//
// #[no_mangle]
// pub extern "C" fn main() -> u32 {
//     unsafe { riot_main() };
//     0
// }
