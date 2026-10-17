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
