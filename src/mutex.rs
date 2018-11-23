use ::core::ops::{Deref, DerefMut};

/// A mutual exclusion primitive useful for protecting shared data
///
/// Unlike the std::sync::Mutex, this has no concept of poisoning, so waiting for mutexes in
/// paniced (and thus locked) threads will lock the accessing thread as well. This is because RIOT
/// threds don't unwind Rust code. As a consequence, the mutex interface is different from the
/// standard library's.
///
/// Several methods (into_inner, get_mut) are not implemented until they're actually needed.
pub struct Mutex<T> {
    mutex: riot_sys::mutex_t,
    data: T,
}

impl<T> Mutex<T> {
    pub fn new(t: T) -> Mutex<T> {
        // FIXME: Expanded version of static function mutex_init
        Mutex { data: t, mutex: riot_sys::mutex_t { queue: riot_sys::list_node_t { next: 0 as *mut _ } } }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        // FIXME here and in try_unlock: Expanded version of static function is used
        unsafe { riot_sys::_mutex_lock(&self.mutex as *const _ as *mut _, 1) };
        MutexGuard { mutex: &self }
    }

    pub fn try_lock(&self) -> Option<MutexGuard<T>> {
        match unsafe { riot_sys::_mutex_lock(&self.mutex as *const _ as *mut _, 0) } {
            1 => Some(MutexGuard { mutex: &self }),
            _ => None,
        }
    }
}

unsafe impl<T: Send> Send for Mutex<T> { }
unsafe impl<T: Send> Sync for Mutex<T> { }

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        unsafe { riot_sys::mutex_unlock(&self.mutex.mutex as *const _ as *mut _) }
    }
}

impl<'a, T> MutexGuard<'a, T> {
    /// Put the current thread to sleep right after unlocking the mutex. This is equivalent to
    /// calling mutex_unlock_and_sleep in RIOT.
    fn unlock_and_sleep(self) {
        let m = &self.mutex.mutex;
        ::core::mem::forget(self);
        unsafe { riot_sys::mutex_unlock_and_sleep(m as *const _ as *mut _) };
    }
}

impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.mutex.data
    }
}

impl<'a, T> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *(&self.mutex.data as *const _ as *mut  _)}
    }
}
