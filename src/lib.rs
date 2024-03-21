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

#![no_std]
#![no_main]

// Well documented code is happy code
#![doc(html_logo_url = ".cargo/dice-purple.png")]
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(rustdoc::missing_doc_code_examples)]


/// Boot routines and specifics
pub mod cpu;

/// Panic
pub mod panic;

/// Hacky QEMU console
pub mod drivers;

/// Syncronization primatives
pub mod sync;
