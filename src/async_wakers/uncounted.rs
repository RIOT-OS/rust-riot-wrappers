//! A more efficient waker implementation that does not count how many clones there are (and
//! happily copies them), at the cost of requiring that the clones may be around forever because
//! the thread for which they are created does not terminate.
//!
//! (If it were allowed to terminate, a waker could be kept exceeding the lifetime, and then the
//! callback would be called even though references it might hold may have become invalid).

use core::task::{RawWaker, RawWakerVTable, Waker};

/// Like [with_counted_waker], but relying on main to never return anyway, there is no need to
/// count.
pub fn with_forever_waker<CB, M>(callback: CB, main: M) -> !
where
    CB: Fn() + Send + Sync,
    M: FnOnce(Waker) -> crate::never::Never,
{
    let dropguard = DropGuard;

    let vtable = &UncountedHelper::<CB>::VTABLE;
    let waker = unsafe { Waker::from_raw(RawWaker::new(&callback as *const CB as *const (), vtable)) };

    main(waker);
}

struct DropGuard;

impl Drop for DropGuard {
    fn drop(&mut self) {
        // Not putting much effort in here -- Rust on RIOT doesn't unwind anyway.
        //
        // (It doesn't so much matter what happens here, more that Drop is implemented so that the
        // dropguard above stays until main (never) returns.)
        loop {
            crate::thread::sleep();
        }
    }
}

struct UncountedHelper<CB>(CB);

// Requiring Send and Sync because the resulting Waker will be Send and Sync.
//
// Not requiring 'static because all clones are necessarily dropped before whatever has the
// callback returns (or its lifetime ends in an endless loop).
impl<CB: Fn() + Send + Sync> UncountedHelper<CB> {
    const VTABLE: RawWakerVTable = RawWakerVTable::new(
        Self::clone,
        Self::wake,
        Self::wake_by_ref,
        Self::drop_waker,
    );

    unsafe fn clone(ptr: *const ()) -> RawWaker {
        // Essentially copy.
        RawWaker::new(ptr, &Self::VTABLE)
    }

    unsafe fn wake(ptr: *const ()) {
        let cb = &*(ptr as *const CB);
        (cb)();
    }

    unsafe fn wake_by_ref(ptr: *const ()) {
        Self::wake(ptr);
    }

    unsafe fn drop_waker(ptr: *const ()) {
    }
}
