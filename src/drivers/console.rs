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
//! # DyseOS Console
//!
//! complete disaster atm.
//!
//!
//! Author: Mitchell Scott <scott.mitchell913@gmail.com>
//!

use crate::sync::mutex::{Mutex, MutexGuard};

//--------------------------------------------------------------------------------------------------
// Public Console Traits
//--------------------------------------------------------------------------------------------------

/// Returns the number of chars written by the Console
/// TODO: Move this somewhere else, or delete it.
pub trait Count {
    /// Default trait implementation returns 0
    fn chars_written(&self) -> usize {
        0
    }
}

/// Full console trait
pub trait Console: core::fmt::Write + Count {}

//--------------------------------------------------------------------------------------------------
// (private) Global instances
//--------------------------------------------------------------------------------------------------

/// SysConsole
///
/// Implements the [Console] trait.
///
/// [_print()] provides a public wrapper to this interface so external projects can use it
/// (thats how its used in [src/start.rs]).
struct SysConsole<const T: usize> {
    chars_written: usize,
}

/// Implementing `core::fmt::Write` enables usage of the `format_args!` macros, which in turn are
/// used to implement the `kernel`'s `print!` and `println!` macros. By implementing `write_str()`,
/// we get `write_fmt()` automatically.
///
/// The function takes an `&mut self`, so it must be implemented for the inner struct.
impl<const T: usize> SysConsole<T> {
    const fn new() -> SysConsole<T> {
        SysConsole { chars_written: 0 }
    }

    /// Send a character.
    fn write_char(&mut self, c: char) {
        unsafe {
            core::ptr::write_volatile(T as *mut u8, c as u8);
        }

        self.chars_written += 1;
    }

}

/// Implementing `core::fmt::Write` enables usage of the `format_args!` macros, which in turn are
/// used to implement the `kernel`'s `print!` and `println!` macros. By implementing `write_str()`,
/// we get `write_fmt()` automatically.
///
/// The function takes an `&mut self`, so it should only be called through dereferencing a thread safe guard.
impl<const T: usize> core::fmt::Write for SysConsole<T> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            // Convert newline to carrige return + newline.
            if c == '\n' {
                self.write_char('\r')
            }

            self.write_char(c);
        }

        Ok(())
    }
}

/// Implements Count
impl<const T: usize> Count for SysConsole<T> {
    /// Returns the number of chars written
    fn chars_written(&self) -> usize {
        self.chars_written
    }
}

/// Implements Console
impl<const T: usize> Console for SysConsole<T> {}

/// A static qemu console implementation wrapped in a mutex for safety
static SYS_CONSOLE_LOCK: Mutex<SysConsole<0x3F20_1000>> = Mutex::new(SysConsole::new());


//--------------------------------------------------------------------------------------------------
// Private api
//--------------------------------------------------------------------------------------------------

/// Return a reference to the qemu console.
///
/// The [crate::sync::mutex::Mutex] is supposed to make this thread safe but isn't ready yet. 
/// Until [crate::sync::mutex] is ready this is not thread safe. Ok to use in Qemu though.
///
/// ## Panics
/// 
///  panic!() is not available because it depends on this. Instead just park the core. Could try
/// waiting for the lock to be available but without poison in [crate::sync::mutex::Mutex] it 
/// might spin forever. Basically this a disaster and probably will be for a bit.
/// 
///  This function parks when the [crate::sync::mutex::Mutex] returns an error.
/// Should probably not park here because currently the mutex does not wait very
/// long. Once futex_wait is available the park should never hit. Until then this will
/// probably park alot.
///
/// ## Examples
///
/// see [crate::drivers::console::_print()]
fn console<'a>() -> MutexGuard<'a, impl Console> {
    match SYS_CONSOLE_LOCK.lock() {
        Ok(guard) => guard,
        Err(_) => crate::cpu::_park(),
    }
}

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

/// Base print implementation
///
/// Uses console() to init a backend that provides the classic rust print frontend. Users should
/// call [crate::print] or [crate::println] not this.
pub fn _print(args: core::fmt::Arguments) {
    // (&mut *console()).write_fmt(args).unwrap();
    core::fmt::Write::write_fmt(&mut *console(), args).unwrap();
}

/// # Print macro
///
/// Prints without a newline.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::drivers::console::_print(format_args!($($arg)*)));
}

/// # Println macro
///
/// Prints with a newline.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        $crate::drivers::console::_print(format_args!($($arg)*));
        $crate::print!("\n");
    })
}
