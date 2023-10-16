//! A waker implementation that keeps count of its clones and panics if any clones were leaked.
//!
//! This is similar in use cases to the `waker-fn` crate, and trades away the alloc dependency,
//! paying with the possibility of panicking at a leak.
//! Panicking at termination is particularly not an issue when tasks never return anyway.
//!
//! This could conceivably be taken out of riot-wrappers into a standalone crate -- but then
//! DropGuard needs to be generalized.
//!
//! ## Warning
//!
//! This is simply not implemented; see [crate::async_wakers::debugging] for steps toward adding
//! it.

use core::task::Waker;

/// Build a [core::task::Waker] from the `callback` (which gets called whenever the waker is
/// woken), and pass it into the main function.
///
/// When the main function exits and clones of the waker are still around, a panic is triggered
/// that ensures that the stack allocated waker (which may still be woken by leaked clone that was
/// stored) stays allocated.
pub fn with_counted_waker<CB, M, R>(callback: CB, main: M) -> R
where
    CB: Fn() + Send + Sync,
    M: FnOnce(Waker) -> R
{
    // Don't forget to add an unwind guard, eg. as in replace_with.
    unimplemented!()
}
