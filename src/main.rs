#![no_std] // don't link Rust std lib
#![no_main] // disable all Rust entry points

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/**
 * entry point to program, default for most systems
 * 'extern "C"' tells compiler to use C-style fxn calling
 * '!' return type is: don't return
 * linker looks for a fxn name `_start` by default
 */
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
#[unsafe(no_mangle)] // don't mangle name of '_start' fxn
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    loop {}
}
