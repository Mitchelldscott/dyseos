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
//! Provide a base implementation for deriving [crate::print] and [crate::println].
//! Also used by [crate::panic] to print info. Currently only setup for qemu raspi3b.
//!
//! Author: Mitchell Scott <scott.mitchell913@gmail.com>
//!

use core::{
    ptr::write_volatile,
    fmt::{Result, Write, Arguments},
};

/// Console for printing
///
/// Stores nothing but implements the [core::fmt::Write] trait and the required 
/// [Console::write_str()] function. The print functions use this to write data, the 
/// implementation defines where the data will end up. The console is public, so everything in 
/// [crate] can use it. [_print()] provides a public interface to this.
struct Console {}

impl Write for Console {
    fn write_str(&mut self, s: &str) -> Result {
        for c in s.chars() {
            unsafe {
                write_volatile(0x3F20_1000 as *mut u8, c as u8);
            }
        }

        Result::Ok(())
    }
}

/// Return a reference to the console.
fn console() -> Console {
    Console {}
}

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

/// Base print implementation, uses the consoles trait to provide the
/// classic rust print interface.
pub fn _print(args: Arguments) {

    console().write_fmt(args).unwrap();

}

/// # Print macro
///
/// Prints without a newline.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console::_print(format_args!($($arg)*)));
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
        $crate::console::_print(format_args!($($arg)*));
        $crate::print!("\n");
    })
}
