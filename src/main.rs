#![no_std] // don't link the rust standard library
#![no_main] // disable Rust-level entry points

use core::{isize, panic::PanicInfo};

static HELLO: &[u8] = b"Hello World!";

#[unsafe(no_mangle)] // don't mangle the name of this function as it needs to stay as _start
pub extern "C" fn _start() -> ! {
    // This is our entry point, beacuse the linker
    // loads a function called _start by default
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

/// This is the function that gets called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
