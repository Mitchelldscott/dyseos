use core::{
    ptr::write_volatile,
    fmt::{Result, Write, Arguments},
};

pub struct Console {
    dst: *mut u8,
}

impl Write for Console {
    fn write_str(&mut self, s: &str) -> Result {
        for c in s.chars() {
            unsafe {
                write_volatile(self.dst, c as u8);
            }
        }

        Result::Ok(())
    }
}

/// Return a reference to the console.
pub fn console() -> Console {
    Console {
        dst: 0x3F20_1000 as *mut u8,
    } // what is this value...
}

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

#[doc(hidden)]
pub fn _print(args: Arguments) {

    console().write_fmt(args).unwrap();

}

/// Prints without a newline.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console::_print(format_args!($($arg)*)));
}

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
