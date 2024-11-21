#![no_std] // don't link Rust std lib
#![no_main] // disable all Rust entry points

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";
static TEXT_COLOR: u8 = 0xb;

// fxn called on Panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

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

    let vga_buffer = 0xb8000 as *mut u8; // raw pointer

    // iterate over HELLO, assign string and color bytes to th buffer
    for (i, &byte) in HELLO.iter().enumerate() {
        // unsafe used b/c of modifying bytes directly
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = TEXT_COLOR;
        }
    }

    loop {}
}
