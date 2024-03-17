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
unsafe fn _kernel_init() -> ! {

    println!("Kernel initializing: ...");
    panic!("Reached end of existing kernel... more coming soon!");

}

