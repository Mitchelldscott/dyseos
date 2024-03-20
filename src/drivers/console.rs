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

pub trait Count {
    fn chars_written(&self) -> usize {
        0
    }
}

pub trait Console: core::fmt::Write + Count {}

//--------------------------------------------------------------------------------------------------
// (private) Global instances
//--------------------------------------------------------------------------------------------------

/// SysConsole
///
/// Implements the trait [Console].
///
/// [_print()] provides a public wrapper to this interface so external projects can use it
/// (thats how its used in ['start.rs']).
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
/// 
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

impl<const T: usize> Count for SysConsole<T> {
    fn chars_written(&self) -> usize {
        self.chars_written
    }
}

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
        Err(_) => crate::_park(),
    }
}

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

/// Base print implementation
///
/// Uses a [SysConsole] to provide the classic rust print interface. Users should
/// call [crate::print] or [crate::println] not this.
/// 
/// ### Disassembly 
/// 
/// ```
/// 0000000000080574 <dyseos::drivers::console::_print::h6de4d1ba3fb6ca41>:
///    80574: d10183ff     	sub	sp, sp, #0x60
///    80578: a9054ffe     	stp	x30, x19, [sp, #0x50]
///    8057c: aa0003e2     	mov	x2, x0
///    80580: 2a1f03e8     	mov	w8, wzr
///    80584: 52800029     	mov	w9, #0x1                // =1
///    80588: d503201f     	nop
///    8058c: 1007d3b3     	adr	x19, 0x90000 <_sbss>
///    80590: 14000002     	b	0x80598 <dyseos::drivers::console::_print::h6de4d1ba3fb6ca41+0x24>
///    80594: d5033f5f     	clrex
///    80598: 7101911f     	cmp	w8, #0x64
///    8059c: 54000200     	b.eq	0x805dc <dyseos::drivers::console::_print::h6de4d1ba3fb6ca41+0x68>
///    805a0: 11000508     	add	w8, w8, #0x1
///    805a4: 085ffe6a     	ldaxrb	w10, [x19]
///    805a8: 35ffff6a     	cbnz	w10, 0x80594 <dyseos::drivers::console::_print::h6de4d1ba3fb6ca41+0x20>
///    805ac: 080a7e69     	stxrb	w10, w9, [x19]
///    805b0: 35ffffaa     	cbnz	w10, 0x805a4 <dyseos::drivers::console::_print::h6de4d1ba3fb6ca41+0x30>
///    805b4: 90000080     	adrp	x0, 0x90000 <_sbss>
///    805b8: 91002000     	add	x0, x0, #0x8
///    805bc: d503201f     	nop
///    805c0: 10019201     	adr	x1, 0x83800 <_phys_bin_start+0x3800>
///    805c4: 940002ca     	bl	0x810ec <core::fmt::write::h8d79d195191be03f>
///    805c8: 37000280     	tbnz	w0, #0x0, 0x80618 <dyseos::drivers::console::_print::h6de4d1ba3fb6ca41+0xa4>
///    805cc: 089ffe7f     	stlrb	wzr, [x19]
///    805d0: a9454ffe     	ldp	x30, x19, [sp, #0x50]
///    805d4: 910183ff     	add	sp, sp, #0x60
///    805d8: d65f03c0     	ret
///    805dc: 91013fe8     	add	x8, sp, #0x4f
///    805e0: d503201f     	nop
///    805e4: 100008a9     	adr	x9, 0x806f8 <<dyseos::sync::mutex::LockError as core::fmt::Debug>::fmt::h538e09299e99be00>
///    805e8: d503201f     	nop
///    805ec: 1001922a     	adr	x10, 0x83830 <_phys_bin_start+0x3830>
///    805f0: 5280002b     	mov	w11, #0x1               // =1
///    805f4: d503201f     	nop
///    805f8: 10019301     	adr	x1, 0x83858 <_phys_bin_start+0x3858>
///    805fc: a903a7e8     	stp	x8, x9, [sp, #0x38]
///    80600: 9100e3e8     	add	x8, sp, #0x38
///    80604: 910023e0     	add	x0, sp, #0x8
///    80608: a900afea     	stp	x10, x11, [sp, #0x8]
///    8060c: a9027feb     	stp	x11, xzr, [sp, #0x20]
///    80610: f9000fe8     	str	x8, [sp, #0x18]
///    80614: 9400010d     	bl	0x80a48 <core::panicking::panic_fmt::hc580a36ad1b33f2e>
///    80618: d503201f     	nop
///    8061c: 100192a0     	adr	x0, 0x83870 <_phys_bin_start+0x3870>
///    80620: d503201f     	nop
///    80624: 100193e3     	adr	x3, 0x838a0 <_phys_bin_start+0x38a0>
///    80628: d503201f     	nop
///    8062c: 100194a4     	adr	x4, 0x838c0 <_phys_bin_start+0x38c0>
///    80630: 91013fe2     	add	x2, sp, #0x4f
///    80634: 52800561     	mov	w1, #0x2b               // =43
///    80638: 9400012d     	bl	0x80aec <core::result::unwrap_failed::h4c3412a30d652e82>
/// ```
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
