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
//! # DyseOS
//!
//! Library with bsp and boot routines for the RaspberryPi-3b,
//! more boards coming much later. The source is almost entirely
//! copied from the rust tutorials by Andre Richter.
//!
//! Author: Mitchell Scott <scott.mitchell913@gmail.com>
//!
//! ## Resources:
//!
//!   - <https://github.com/embedded-rust/rust/raspberrypi-OS-tutorials.git>
//!

// Well documented code is happy code
#![warn(missing_docs)]
// None of that S#!t
#![no_std]
#![no_main]

// will need a feature for this when more boards are added
use aarch64_cpu::asm;
// This is cool but the asm script seems more productive
use core::arch::asm;

/// Hacky QEMU console
pub mod drivers;
/// Panic
pub mod panic;
/// Syncronization primatives
pub mod sync;

/// # Start code
///
/// If on the boot core starts the kernel, if not parks it.
/// Also initializes the bss section by calling [_init_mem()]. This code is
/// linked to the beggining of the .text section by the linker script.
///
/// ### TODO:
/// - Make board independant using features
/// - Learn about processors to see if it can do more
///
/// ### Disassembly
/// ```
/// 0000000000080000 <_start>:
///    80000: d53800a0     	mrs	x0, MPIDR_EL1
///    80004: 92400400     	and	x0, x0, #0x3
///    80008: aa1f03e1     	mov	x1, xzr
///    8000c: eb01001f     	cmp	x0, x1
///    80010: 54000201     	b.ne	0x80050 <_park>
///    80014: d503201f     	nop
///    80018: 10ffff40     	adr	x0, 0x80000 <_start>
///    8001c: 9100001f     	mov	sp, x0
///    80020: d503201f     	nop
///    80024: 100001be     	adr	x30, 0x80058 <_kernel_init>
///    80028: 14000003     	b	0x80034 <_init_mem>
///    8002c: d503205f     	wfe
///    80030: 17ffffff     	b	0x8002c <_start+0x2c>
/// ```
#[link_section = ".text._start"]
#[no_mangle]
unsafe fn _start() -> ! {
    asm!(
        // check if this is the boot core (ID = 0)
        "mrs    x0, mpidr_el1",
        "and    x0, x0, #0x3",
        "mov    x1, xzr",
        "cmp    x0, x1",
        "b.ne   _park",
        // load boot stack to x0 then sp
        "adrp	x0, _ebcstack",
        "add	x0, x0, #:lo12:_ebcstack",
        "mov	sp, x0",
        // set x30 to kernel init
        "adrp	x30, _kernel_init",
        "add	x30, x30, #:lo12:_kernel_init",
        // // begin init bss
        "b      _init_mem"
    );

    _park();
}

/// Initialize the bss section of memory
///
/// Copied directly from <https://docs.rust-embedded.org/embedonomicon/main.html#life-before-main>
///
/// Requires that _ebss & _sbss are defined by the linker. It is UB to call when
/// _ebss < _sbss
///
/// ### Disassembly
/// ```
///
/// 0000000000080034 <_init_mem>:
///    80034: d503201f     	nop
///    80038: 1007fe40     	adr	x0, 0x90000 <_sbss>
///    8003c: d503201f     	nop
///    80040: 1007fe88     	adr	x8, 0x90010 <_ebss>
///    80044: cb000102     	sub	x2, x8, x0
///    80048: 2a1f03e1     	mov	w1, wzr
///    8004c: 14000d15     	b	0x834a0 <memset>
///
/// ```
#[link_section = ".text._init_mem"]
#[no_mangle]
unsafe fn _init_mem() {
    extern "C" {
        static mut _sbss: u8;
        static _ebss: u8;
    }

    let count = &_ebss as *const u8 as usize - &_sbss as *const u8 as usize;
    core::ptr::write_bytes(&mut _sbss as *mut u8, 0, count);
}

/// Park the core
///
/// loops and waits for an event.
///
/// ### Disassembly
/// ```
///
/// 0000000000080054 <_park>:
///    80054: d503205f     	wfe
///    80058: 17ffffff     	b	0x80054 <_park>
///
/// ```
#[link_section = ".text._park"]
#[no_mangle]
pub fn _park() -> ! {
    loop {
        asm::wfe();
    }
}
