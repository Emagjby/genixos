#![no_std] // don't link the rust standard library
#![no_main] // disable Rust-level entry points

mod vga;

use core::panic::PanicInfo;

#[unsafe(no_mangle)] // don't mangle the name of this function as it needs to stay as _start
pub extern "C" fn _start() -> ! {
    // This is our entry point, beacuse the linker
    // loads a function called _start by default

    let mut writer = vga::VgaWriter::new();

    writer.write_startup_message();

    loop {}
}

/// This is the function that gets called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
