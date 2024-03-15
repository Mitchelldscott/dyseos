use core::{
    ptr::write_volatile,
    fmt::{Result, Write},
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
pub fn qemu_console() -> Console {
    Console {
        dst: 0x3F20_1000 as *mut u8,
    } // what is this value...
}
