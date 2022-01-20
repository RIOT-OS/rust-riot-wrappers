//! Tools for providing a RIOT main function
//!
//! The main contribution of this module is the [riot_main] macro.
//!
//! The alternative to using that (other than doing it manually) is to have C code along with the
//! Rust application that occupies the main function.
//!
//! In these cases, Rust code can be called into from the main C code by declaring the entry
//! functions `#[no_mangle] pub extern "C"`, and having analogous `extern` functions in the calling
//! C code.

use crate::stdio::println;

// General alternative to this module: Build the extern "C" main all the time and request that the
// application implement a named function. I never got the main function to be carried to the
// linker step, though. If implemented like this, the module needs to be gated like
// set_panic_handler.
//
// extern "Rust" {
//     fn riot_main();
// }
//
// #[no_mangle]
// pub extern "C" fn main() -> u32 {
//     unsafe { riot_main() };
//     0
// }

use core::fmt;

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
        pub extern "C" fn c_main() -> i32 {
            use riot_wrappers::main::Termination;
            $main().report()
        }
    };
}

#[macro_export]
macro_rules! riot_main_with_tokens {
    ($main:ident) => {
        #[export_name = "main"]
        pub extern "C" fn c_main() -> i32 {
            // unsafe: By construction of the C main function this only happens at startup time
            // with a thread that hasn't done anything relevant before.
            let unique = unsafe { riot_wrappers::thread::StartToken::new() };

            let (result, token): (_, riot_wrappers::thread::TerminationToken) = $main(unique);
            use riot_wrappers::main::Termination;
            result.report()
        }
    };
}

/// A result trait for main methods, analogous to std::process::Termination
pub trait Termination {
    fn report(self) -> i32;
}

impl Termination for () {
    fn report(self) -> i32 {
        0
    }
}

impl Termination for i32 {
    fn report(self) -> i32 {
        self
    }
}

// Copied and stripped down from std

impl<E: fmt::Debug> Termination for Result<(), E> {
    fn report(self) -> i32 {
        match self {
            Ok(()) => ().report(),
            Err(err) => Err::<crate::Never, _>(err).report(),
        }
    }
}

impl Termination for crate::Never {
    fn report(self) -> i32 {
        self
    }
}

impl Termination for core::convert::Infallible {
    fn report(self) -> i32 {
        match self {}
    }
}

impl<E: fmt::Debug> Termination for Result<crate::Never, E> {
    fn report(self) -> i32 {
        match self {
            Err(err) => {
                println!("Error: {:?}", err);
                1
            }
            Ok(never) => never,
        }
    }
}
