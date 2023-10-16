//! A simple stupid debug helper that's way more unsafe than I'd ever like to see code
//!
//! Essentially this crates a bump allocator in which wakers are kept forever -- but their indices
//! are convenient names for them.

use core::task::{RawWaker, RawWakerVTable, Waker};
use core::mem::MaybeUninit;

const POOLSIZE: usize = 100;

pub fn with_waker<CB, M>(callback: CB, main: M) -> !
where
    CB: Fn() + Send + Sync,
    M: FnOnce(Waker) -> crate::never::Never,
{
    let dropguard = DropGuard;

    // We should probably pin this, but what could possibly go wrong if we don't...
    let mut central = CentralWaker { pool: core::mem::MaybeUninit::uninit_array(), callback, count: crate::mutex::Mutex::new(1) };
    let central_ptr = &central as *const _;
    for waker in central.pool.iter_mut() {
        waker.write(DebugWaker { central: central_ptr });
    }
    let root_waker = unsafe { central.pool[0].assume_init_ref() };

    let vtable = &DebugWaker::<CB>::VTABLE;
    let waker = unsafe { Waker::from_raw(RawWaker::new(root_waker as *const DebugWaker<CB> as *const (), vtable)) };

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

struct CentralWaker<CB: Fn() + Send + Sync> {
    count: crate::mutex::Mutex<usize>,
    pool: [MaybeUninit<DebugWaker<CB>>; POOLSIZE],
    callback: CB,
}

impl<CB: Fn() + Send + Sync> CentralWaker<CB> {
    fn new_item(&self) -> &DebugWaker<CB> {
        let mut count = self.count.lock();
        let ret = *count;
        assert!(ret < POOLSIZE, "Capacity exceeded");
        *count = ret + 1;
        return unsafe { self.pool[ret].assume_init_ref() };
    }
}

struct DebugWaker<CB: Fn() + Send + Sync> {
    central: *const CentralWaker<CB>,
    // The label is implicit, it's the offset in the pool
}

// Requiring Send and Sync because the resulting Waker will be Send and Sync.
//
// Not requiring 'static because all clones are necessarily dropped before whatever has the
// callback returns (or its lifetime ends in an endless loop).
impl<CB: Fn() + Send + Sync> DebugWaker<CB> {
    fn central(&self) -> &CentralWaker<CB> {
        // They're assumed to be initialized well
        unsafe { &*self.central }
    }

    fn index(&self) -> usize {
        self.central().pool.iter().position(|slot| unsafe { slot.assume_init_ref() } as *const _ == self as *const _).unwrap()
    }

    const VTABLE: RawWakerVTable = RawWakerVTable::new(
        Self::clone,
        Self::wake,
        Self::wake_by_ref,
        Self::drop_waker,
    );

    unsafe fn clone(ptr: *const ()) -> RawWaker {
        let s = &*(ptr as *const Self);
        let next = s.central().new_item();
        crate::println!("DW: Cloning {} into {}", s.index(), next.index());
        RawWaker::new(next as *const Self as *const (), &Self::VTABLE)
    }

    unsafe fn wake(ptr: *const ()) {
        let s = &*(ptr as *const Self);
        crate::println!("Waking {}", s.index());
        (s.central().callback)();
    }

    unsafe fn wake_by_ref(ptr: *const ()) {
        let s = &*(ptr as *const Self);
        crate::println!("Waking through reference to {}", s.index());
        (s.central().callback)();
    }

    unsafe fn drop_waker(ptr: *const ()) {
        let s = &*(ptr as *const Self);
        crate::println!("Dropping {}", s.index());
    }
}
