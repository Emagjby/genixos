pub unsafe fn disable_interrupts() {
    unsafe {
        core::arch::asm!("cli", options(nostack));
    }
}

pub unsafe fn enable_interrupts() {
    unsafe {
        core::arch::asm!("sti", options(nostack));
    }
}
