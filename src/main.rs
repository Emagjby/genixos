#![no_std] // don't link the rust standard library
#![no_main] // disable Rust-level entry points

mod arch;
mod console;
mod diagnostics;
mod drivers;
mod sync;

use core::panic::PanicInfo;

#[unsafe(no_mangle)] // don't mangle the name of this function as it needs to stay as _start
pub extern "C" fn _start() -> ! {
    // This is our entry point, beacuse the linker
    // loads a function called _start by default
    drivers::init();

    println!("Loading GenixOS...");

    cprintln!(Red; "Hi!");
    cprintln!(Red, Green; "Hi!");

    loop {}
}

/// This is the function that gets called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    diagnostics::panic::report_panic(info);
    loop {}
}
