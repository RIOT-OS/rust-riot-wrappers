// Sketches for a task manager:
//
// Tasks are bound to a process (probably requiring that the task manager would not exit and thus
// is &'static, that just lets us skip so much otherwise necessary cleanup code and maybe even
// tracking).
//
// Different tasks are preprovisioned into the runner, maybe in a builder pattern. Tasks need to
// have a flag associated with them, by which that task would be woken up. Generally using
// different flags is nice b/c it means that when one task's flag is woken, the other task doesn't
// need to go through calls that we know to be Pending anyway -- but using the same flag for
// multiple tasks might be nice as it'd allow things that don't differentiate event clearings to
// both run. (Example: Async UDP socket callbacks don't distinguish read from write; so we couldn't
// practically have the sender in one task and the receiver in another, at least not if UDP sending
// blocks).
//
// Setting callbacks would happen in the waker. A single callback should suffice (it doesn't need
// to be registered while it hasn't returned WouldBlock again), and would be good to have b/c
// otherwise a socket that was once used in a task will keep waking that task. Clearing the
// callback in the callback would be one option. An advanced version would be having the flag in
// the CB data as a bit set, so that different tasks in the same thread could be listening for a
// callback. (Either way, the CB would clear itself and set the thread flags). For setting the
// callbacks, it might make sense to assert that nobody else is listening in (or in the advanced
// version, only the local PID), that might need a _get_cb or just reaching out into the callback
// attributes.
//
// Mapping in mutexes (as to have `mutex.lock().await`) would be great for show-off, but tricky, as
// mutexes are a very slim list that just relies on waking up the process. It might need manual
// placing of the thread in the mutex chain, and polling all tasks. The waiter would be allocated
// in the coroutine, and rely on that being pinned. Not sure what'd happen if different tasks (or
// different places in the same task) tried locking at the same time -- there'd be different
// waiters from the same thread, probably that'd just work. Some care is needed around cancellation
// -- we'd need a Drop guarantee over the nascent lock, and remove it from the queue (through
// cancellable mutex) when e.g. another branch of a "whichever thing asyncs first" condition is
// taken, and  that state would be dropped.

use core::pin::Pin;

use pin_project::pin_project;

#[pin_project]
pub struct Task<F, T, const FLAG: u8 = 0>
where
    F: core::future::Future<Output = T>
{
    #[pin]
    future: F,
}

impl<F, T, const FLAG: u8> Task<F, T, FLAG>
where
    F: core::future::Future<Output = T>
{
    pub fn new(future: F) -> Self {
        Task { future }
    }

    pub fn run_to_completion(self: Pin<&mut Self>) -> T {
        // Would be nice, but needs alloc
        // let waker = waker_fn::waker_fn(|| crate::println!("woken"));

        // FIXME: Should use the counted one as we advertise that we can return
        
        let our_pid = crate::thread::get_pid();

        crate::async_wakers::debugging::with_waker(
//         crate::async_wakers::uncounted::with_forever_waker(
            // FIXME: if we stick with non-counting, we don't even need a waker function, we could
            // build a wakter whose only payload is the packed struct PID / flag number. Sadly,
            // thus we only spare ourselves the one indirection and the allocation of the waker
            // here, but not the size of the waker because internally to Waker it's still a manual
            // `&dyn RawWaker`, which needs its two words.
            || {
                let our_thread = unsafe  { riot_sys::thread_get(our_pid.into()) };
                // Would be the safer conversion, but it's not public in riot-wrappers.
                // let thread = riot_wrappers::inline_cast_mut(thread);
                let our_thread = our_thread as _;
                unsafe { riot_sys::thread_flags_set(our_thread, 1 << FLAG) };
            },
            |waker| {
                let mut ctx = core::task::Context::from_waker(&waker);
                let mut future: Pin<&mut F> = self.project().future;
                let future: &mut Pin<&mut F> = &mut future;

                loop {
                    match future.as_mut().poll(&mut ctx) {
                        core::task::Poll::Ready(t) => {
                            // FIXME see above
                            unimplemented!("Task returned.");
                            // return t;
                        }
                        core::task::Poll::Pending => ()
                    }

                    unsafe { riot_sys::thread_flags_wait_any(1 << FLAG) };
                }
            },
            )
    }
}
