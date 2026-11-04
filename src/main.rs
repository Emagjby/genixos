#![no_std] // don't link the rust standard library
#![no_main] // disable Rust-level entry points
#![feature(abi_x86_interrupt)]

mod arch;
mod collections;
mod console;
mod diagnostics;
mod drivers;
mod memory;
mod sync;
mod time;

use bootloader::BootInfo;
use core::panic::PanicInfo;

use crate::arch::x86_64::interrupts;

#[unsafe(no_mangle)] // don't mangle the name of this function as it needs to stay as _start
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    // This is our entry point, beacuse the linker
    // loads a function called _start by default
    drivers::init();

    let consumer = drivers::ps2::initialize();

    interrupts::load();
    memory::inspect(boot_info);

    console::clear();
    println!("Loading GenixOS...");

    loop {
        // check keystrokes by emptying the scancode queue if it has elements
        while let Some(scancode) = consumer.pop() {
            println!("Scancode: {:#04x}", scancode);
        }
    }

    loop {}
}

/// This is the function that gets called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    diagnostics::panic::report_panic(info);
    loop {}
}
