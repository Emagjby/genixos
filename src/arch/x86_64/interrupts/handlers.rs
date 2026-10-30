use crate::arch::x86_64::interrupts;

#[repr(C)]
#[derive(Debug)]
pub struct InterruptStackFrame {
    pub instruction_pointer: u64,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: u64,
    pub stack_segment: u64,
}

pub extern "x86-interrupt" fn breakpoint_handler(_frame: InterruptStackFrame) {
    crate::debug!("Breakpoint: vector 3 triggered\n");
}

pub extern "x86-interrupt" fn double_fault_handler(
    frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    use crate::drivers::serial::try_with_serial1;
    use core::fmt::Write;

    try_with_serial1(|s| {
        let _ = s.write_fmt(format_args!("{:?}\n{:?}", frame, error_code));
    });
    panic!("{:?}\n{:?}", frame, error_code)
}

pub extern "x86-interrupt" fn timer_interrupt_handler(_frame: InterruptStackFrame) {
    interrupts::tick_increment();
    interrupts::pic_send_eoi(0);
}
