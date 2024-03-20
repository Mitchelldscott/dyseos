/********************************************************************************
 *
 *      ____                     ____          __           __       _
 *     / __ \__  __________     /  _/___  ____/ /_  _______/ /______(_)__  _____
 *    / / / / / / / ___/ _ \    / // __ \/ __  / / / / ___/ __/ ___/ / _ \/ ___/
 *   / /_/ / /_/ (__  )  __/  _/ // / / / /_/ / /_/ (__  ) /_/ /  / /  __(__  )
 *  /_____/\__, /____/\___/  /___/_/ /_/\__,_/\__,_/____/\__/_/  /_/\___/____/
 *        /____/
 *
 *
 ********************************************************************************/
//! # DyseOS Panic handler
//!
//! Author: Mitchell Scott <scott.mitchell913@gmail.com>
//!

/// Stop immediately if called a second time.
///
/// Copied from <https://github.com/embedded-rust/rust/raspberrypi-OS-tutorials.git>
///
/// ## Note
///
/// Using atomics here relieves us from needing to use `unsafe` for the static variable.
///
/// On `AArch64`, which is the only implemented architecture at the time of writing this,
/// [`AtomicBool::load`] and [`AtomicBool::store`] are lowered to ordinary load and store
/// instructions. They are therefore safe to use even with MMU + caching deactivated.
///
/// [`AtomicBool::load`]: core::sync::atomic::AtomicBool::load
/// [`AtomicBool::store`]: core::sync::atomic::AtomicBool::store
#[no_mangle]
fn panic_prevent_reenter() {
    use core::sync::atomic::{AtomicBool, Ordering};

    #[cfg(not(target_arch = "aarch64"))]
    compile_error!("Add the target_arch to above's check if the following code is safe to use");

    static PANIC_IN_PROGRESS: AtomicBool = AtomicBool::new(false);

    if !PANIC_IN_PROGRESS.load(Ordering::Relaxed) {
        PANIC_IN_PROGRESS.store(true, Ordering::Relaxed);

        return;
    }

    crate::_park()
}

/// # Panic handler
///
/// Mostly copied from <https://github.com/embedded-rust/rust/raspberrypi-OS-tutorials.git> but
/// removed the unstable feature use.
///
/// When [panic!()] is called information on the thread is packaged into
/// [core::panic::PanicInfo]. This handler prints the panic location and
/// and then parks the core.
///
/// ## TODO:
/// - Accessing the panic message is an unstable feature, the current workaround
/// is to use [core::fmt::Debug] to print the full [core::panic::PanicInfo]
///
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // Protect against panic infinite loops if any of the following code panics itself.
    panic_prevent_reenter();

    let (location, line) = match info.location() {
        Some(loc) => (loc.file(), loc.line()),
        _ => ("???", 0),
    };

    crate::println!("Kernel panicked at {}:{}\n{:?}", location, line, info,);

    crate::_park();
}
