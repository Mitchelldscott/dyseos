#![no_std]
#![no_main]

use aarch64_cpu::asm;
use core::arch::asm;

pub mod console;
pub mod print;

pub mod panic;

// The reset handler
#[link_section = ".text"]
#[no_mangle]
unsafe fn _start() -> ! {
    asm!(
        "mrs    x0, mpidr_el1",
        "and    x0, x0, #0x3",
        "mov    x1, xzr",
        "cmp    x0, x1",
        "b.ne   _park",
        "adrp	x0, __boot_core_stack_end_exclusive",
        "add	x0, x0, #:lo12:__boot_core_stack_end_exclusive",
        "mov	sp, x0",
        "adrp	x0, __bss_start",
        "add	x0, x0, #:lo12:__bss_start",
        "adrp	x1, __bss_end_exclusive",
        "add	x1, x1, #:lo12:__bss_end_exclusive",
        "add	x1, x1, #0x10",
        "adrp	x30, _kernel_init",
        "add	x30, x30, #:lo12:_kernel_init",
        "b	    _init_section",
    );

    loop {}
}

#[link_section = ".text"]
#[no_mangle]
unsafe fn _init_section() {
    asm!(
        "stp	xzr, xzr, [x0], #16",
        "cmp    x0, x1",
        "b.ne   _init_section",
    );
}

#[link_section = ".text"]
#[no_mangle]
pub fn _park() -> ! {
    loop {
        asm::wfe();
    }
}