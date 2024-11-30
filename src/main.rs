#![no_std] // don't link Rust std lib
#![no_main] // disable all Rust entry points

use core::panic::PanicInfo;

// Rust module import
mod vga_buffer;

/**
 * unsafe:
 * - still allows safety checks
 * - tells compiler ops are valid
 * - actions possible:
 *  - dereference raw pointer
 *  - call unsafe fxn or mthd
 *  - access/modify static var
 *  - implement unsafe trait
 *  - access fields of `union`
 */

#[no_mangle] // don't mangle name of fxn
pub extern "C" fn _start() -> ! {
    // entry point to program, default for most systems
    // linker looks for a fxn name `_start` by default
    println!("Hello World{}", "!");

    loop {}

    // some panic message (for example)
    // panic!("Some panic message");
}

// fxn called on Panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
