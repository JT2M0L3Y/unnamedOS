#![no_std] // don't link Rust std lib
#![no_main] // disable all Rust entry points

use core::panic::PanicInfo;

// fxn called on Panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // don't mangle name of fxn
pub extern "C" fn _start() -> ! {
    // entry point to program, default for most systems
    // linker looks for a fxn name `_start` by default
    loop {}
}
