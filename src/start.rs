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
//! # DyseOS Kernel
//!
//! Implementation of a Linux Kernel.
//!
//! Author: Mitchell Scott <scott.mitchell913@gmail.com>
//!

#![no_std]
#![no_main]

use dyseos::*;

#[no_mangle]
/// Initialize the kernel
///
///
/// ### Disassembly
/// 
/// ```
/// 0000000000080058 <_kernel_init>:
///   80058: d10143ff     	sub	sp, sp, #0x50
///   8005c: a9044ff4     	stp	x20, x19, [sp, #0x40]
///   80060: d503201f     	nop
///   80064: 1001aae8     	adr	x8, 0x835c0 <_phys_bin_start+0x35c0>
///   80068: 52800033     	mov	w19, #0x1               // =1
///   8006c: d503201f     	nop
///   80070: 1001ab14     	adr	x20, 0x835d0 <_phys_bin_start+0x35d0>
///   80074: 910003e0     	mov	x0, sp
///   80078: f9001bfe     	str	x30, [sp, #0x30]
///   8007c: a901ffff     	stp	xzr, xzr, [sp, #0x18]
///   80080: a9004fe8     	stp	x8, x19, [sp]
///   80084: f9000bf4     	str	x20, [sp, #0x10]
///   80088: 9400013b     	bl	0x80574 <dyseos::drivers::console::_print::h6de4d1ba3fb6ca41>
///   8008c: d503201f     	nop
///   80090: 1001aa48     	adr	x8, 0x835d8 <_phys_bin_start+0x35d8>
///   80094: 910003e0     	mov	x0, sp
///   80098: a901ffff     	stp	xzr, xzr, [sp, #0x18]
///   8009c: f9000bf4     	str	x20, [sp, #0x10]
///   800a0: a9004fe8     	stp	x8, x19, [sp]
///   800a4: 94000134     	bl	0x80574 <dyseos::drivers::console::_print::h6de4d1ba3fb6ca41>
///   800a8: d503201f     	nop
///   800ac: 1001aba8     	adr	x8, 0x83620 <_phys_bin_start+0x3620>
///   800b0: d503201f     	nop
///   800b4: 1001ac61     	adr	x1, 0x83640 <_phys_bin_start+0x3640>
///   800b8: 910003e0     	mov	x0, sp
///   800bc: a901ffff     	stp	xzr, xzr, [sp, #0x18]
///   800c0: a9004fe8     	stp	x8, x19, [sp]
///   800c4: f9000bf4     	str	x20, [sp, #0x10]
///   800c8: 94000260     	bl	0x80a48 <core::panicking::panic_fmt::hc580a36ad1b33f2e>
/// ```
unsafe fn _kernel_init() -> ! {
    println!("Kernel initializing: ...");
    panic!("Reached end of existing kernel... more coming soon!");
}
