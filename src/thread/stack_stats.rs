/// Gathered information about a thread, returned by [KernelPID::stack_stats()].
///
/// All accessors are unconditional, because the StackStats can't be obtained without develhelp in
/// the first place.
///
/// ## Upgrade warning
///
/// In the next breaking release, together with several deprecations, this will be marked
/// `#[non_exhaustive]`. Start adding a `..` to ensure a good upgrade path.
///
/// (This will pave the way for the addition of the stack pointer through `thread_get_sp`).
#[derive(Debug)]
pub struct StackStats {
    pub(crate) start: *mut i8,
    pub(crate) size: usize,
    pub(crate) free: usize,
}

impl StackStats {
    pub fn start(&self) -> *mut i8 {
        self.start
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn end(&self) -> *mut i8 {
        // This is the last legal pointer to construct on this ... last-plus-one rule.
        unsafe { self.start.offset(self.size as isize) }
    }

    pub fn free(&self) -> usize {
        self.free
    }

    pub fn used(&self) -> usize {
        self.size - self.free
    }
}

#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub enum StackStatsError {
    /// Requested PID does not correspond to a thread
    NoSuchThread,
    /// Details on the stack are unavailable because develhelp is disabled
    InformationUnavailable,
}
