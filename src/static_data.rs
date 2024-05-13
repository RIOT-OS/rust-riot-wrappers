use core::cell::UnsafeCell;
use core::sync::atomic::AtomicBool;

pub struct StaticData<T> {
    taken: AtomicBool,
    data: UnsafeCell<T>,
}

// Self::taken atomically guarantees Self::data can be referenced only once
unsafe impl<T> Sync for StaticData<T> {}

/// Wrapper for safely defining variables with static storage.
///
/// On embedded, large static data structures are often required (e.g. buffers).
/// This is a workaround the borrow checker to safely obtain exactly one reference
/// to mutable static data. Trying to obtain a second reference will panic.
impl<T> StaticData<T> {
    pub const fn new(val: T) -> Self {
        Self {
            data: UnsafeCell::new(val),
            taken: AtomicBool::new(false),
        }
    }

    /// Obtain a mutable reference to the static data. Will panic if called more
    /// than once.
    ///
    /// ## Panics
    ///
    /// # Examples
    ///
    /// ```
    /// static BUF: StaticData<[u8; 8]> = StaticData::new([0; 8]);
    ///
    /// let buf: &'static mut [u8; 8]  = BUF.take();
    /// buf[0] = 99;
    /// assert_eq!(buf[0], 99);
    /// ```
    pub fn take(&'static self) -> &mut T {
        assert!(
            !self.taken.swap(true, core::sync::atomic::Ordering::Acquire),
            "taken more than once!"
        );
        unsafe {
            // Ordering::Acquire above ensures we get to this point iff this
            // is the first reference
            &mut *self.data.get()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn take_once() {
        static BUF: StaticData<[u8; 8]> = StaticData::new([0; 8]);

        let buf = BUF.take();

        buf[0] = 99;
        assert_eq!(buf[0], 99);
    }

    #[test]
    #[should_panic]
    fn take_twice() {
        static BUF: StaticData<[u8; 8]> = StaticData::new([0; 8]);

        let buf1 = BUF.take();
        let buf2 = BUF.take();

        // we should not reach this point
        buf1[0] = 99;
        assert_eq!(buf2[0], 99);
    }
}
