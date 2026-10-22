#[repr(C, packed)]
pub struct TaskStateSegment {
    _r0: u32,
    pub rsp0: u64,
    pub rsp1: u64,
    pub rsp2: u64,
    _r1: u64,
    pub ist1: u64,
    pub ist2: u64,
    pub ist3: u64,
    pub ist4: u64,
    pub ist5: u64,
    pub ist6: u64,
    pub ist7: u64,
    _r2: u64,
    _r3: u16,
    pub iopb: u16,
}

impl TaskStateSegment {
    pub const fn new() -> Self {
        Self {
            _r0: 0,
            rsp0: 0,
            rsp1: 0,
            rsp2: 0,
            _r1: 0,
            ist1: 0,
            ist2: 0,
            ist3: 0,
            ist4: 0,
            ist5: 0,
            ist6: 0,
            ist7: 0,
            _r2: 0,
            _r3: 0,
            iopb: 104,
        }
    }

    pub fn set_ist(&mut self, idx: u8, stack_top: *const u8) {
        assert!(idx > 0 && idx < 8, "IST range is 1-7");

        match idx {
            1 => self.ist1 = stack_top as u64,
            2 => self.ist2 = stack_top as u64,
            3 => self.ist3 = stack_top as u64,
            4 => self.ist4 = stack_top as u64,
            5 => self.ist5 = stack_top as u64,
            6 => self.ist6 = stack_top as u64,
            7 => self.ist7 = stack_top as u64,
            _ => {} // impossible
        }
    }
}

#[repr(C, align(16))]
pub struct DoubleFaultStack([u8; 16 * 1024]);

impl DoubleFaultStack {
    pub const fn new() -> Self {
        Self([0; 16 * 1024])
    }

    pub fn stack_start(&self) -> *const u8 {
        core::ptr::addr_of!(self.0) as *const u8
    }

    pub fn stack_top(&self) -> *const u8 {
        self.stack_start().wrapping_add(16 * 1024)
    }
}
