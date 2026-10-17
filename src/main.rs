#![no_std] // don't link the rust standard library
#![no_main] // disable Rust-level entry points
#![feature(abi_x86_interrupt)]

mod arch;
mod console;
mod diagnostics;
mod drivers;
mod memory;
mod sync;

use bootloader::BootInfo;
use core::panic::PanicInfo;

#[unsafe(no_mangle)] // don't mangle the name of this function as it needs to stay as _start
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    // This is our entry point, beacuse the linker
    // loads a function called _start by default
    drivers::init();
    arch::x86_64::interrupts::load();
    memory::inspect(boot_info);

    console::clear();
    println!("Loading GenixOS...");

    loop {}
}

/// This is the function that gets called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    diagnostics::panic::report_panic(info);
    loop {}
}
