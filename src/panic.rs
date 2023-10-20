#[panic_handler]
fn panic(info: &::core::panic::PanicInfo) -> ! {
    use crate::thread;

    let os_can_continue = crate::thread::InThread::new()
        // Panics with IRQs off are fatal because we can't safely re-enable them
        .map(|i| i.irq_is_enabled())
        // Panics in ISRs are always fatal because continuing in threads would signal to the
        // remaining system that the ISR terminated
        .unwrap_or(false);

    if !os_can_continue {
        // We can't abort on stable -- but even if we could: Set a breakpoint and wait for the
        // fault handler to reboot us of no debugger is attached? Spin endlessly? core_panic should
        // already answer all these questions.

        // Not attempting to print -- it would only get through on devices where stdio is provided
        // by a UART, and with these the debugger is usually also close enough that the risk of
        // smashing things by overflowing the ISR stack outweighs the benefits.

        unsafe {
            riot_sys::core_panic(
                riot_sys::core_panic_t_PANIC_GENERAL_ERROR,
                cstr::cstr!("RUST PANIC").as_ptr() as _,
            )
        };
    } else {
        // I *guess* it's OK for a panic to simply make a thread into a zombie -- this does allow other
        // threads (including spawned Rust threads) to continue, but my layman's understanding of
        // panicking is that that's OK because whatever we were just mutating can simply never be used
        // by someone else ever again.

        let me = thread::get_pid();

        if cfg!(feature = "panic_handler_format") {
            use crate::stdio::println;

            println!(
                "Error in thread {:?} ({}):",
                me,
                me.get_name().unwrap_or("unnamed")
            );
            println!("{}", info);
        } else {
            let mut stdio = crate::stdio::Stdio {};
            use core::fmt::Write;
            let _ = stdio.write_str("Panic in thread ");
            let _ = stdio.write_str(me.get_name().unwrap_or("unnamed"));
            let _ = stdio.write_str("!\n");
        }

        if cfg!(feature = "panic_handler_crash") {
            unsafe {
                riot_sys::core_panic(
                    riot_sys::core_panic_t_PANIC_GENERAL_ERROR,
                    cstr::cstr!("RUST PANIC").as_ptr() as _,
                )
            }
        }

        // Not trying any unwinding -- this thread is just dead, won't be re-claimed, any mutexes it
        // holds are just held indefinitely rather than throwing poison errors.
        loop {
            thread::sleep();
        }
    }
}

// We could make this conditional also on panic="abort" and thus enable stable compilation when
// that flag is enabled -- but then again https://github.com/rust-lang/rust/issues/77443 is not
// stable yet so it's kind of moot (and I don't want to introduce yet another crate feature that'll
// largely be untested).
#[cfg(all(target_arch = "x86", not(panic = "abort")))]
#[lang = "eh_personality"]
fn rust_eh_personality() {
    loop {}
}
