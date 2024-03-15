#![no_std]
#![no_main]

#[allow(unused_imports)]
use dyseos;

#[no_mangle]
pub unsafe fn _kernel_init() -> ! {
    panic!("Reached end of existing kernel... more coming soon!");
}


