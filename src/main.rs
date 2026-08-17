#![no_std]  // don't link the Rust standard library
#![no_main] // disable all Rust-level entry point
use core::panic::PanicInfo;

#[unsafe(no_mangle)]    // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // name '_start' by default
    loop {}
}

// this function called panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

