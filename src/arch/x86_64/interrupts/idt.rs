#[repr(C)]
#[repr(align(16))]
pub struct InterruptDescriptorTable {
    entries: [IdtEntry; 256],
}

impl InterruptDescriptorTable {
    pub const fn new() -> Self {
        Self {
            entries: [IdtEntry::new(); 256],
        }
    }

    pub fn set_handler(&mut self, vector: usize, handler_address: u64) {
        let offset_low: u16 = handler_address as u16;
        let offset_mid: u16 = (handler_address >> 16) as u16;
        let offset_high: u32 = (handler_address >> 32) as u32;

        unsafe {
            self.entries[vector].selector = get_current_cs();
        }
        self.entries[vector].offset_low = offset_low;
        self.entries[vector].offset_middle = offset_mid;

        self.entries[vector].offset_high = offset_high;
        self.entries[vector].type_attributes = 0x8E;
        self.entries[vector].reserved = 0;
        self.entries[vector].ist = 0;
    }

    pub fn load(&'static self) {
        let pointer = IdtPointer {
            limit: (size_of::<[IdtEntry; 256]>() - 1) as u16,
            base: self.entries.as_ptr() as u64,
        };

        unsafe { lidt(pointer) }
    }
}

#[repr(C, packed)]
struct IdtPointer {
    limit: u16,
    base: u64,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct IdtEntry {
    offset_low: u16,
    selector: u16,
    ist: u8,
    type_attributes: u8,
    offset_middle: u16,
    offset_high: u32,
    reserved: u32,
}

impl IdtEntry {
    pub const fn new() -> Self {
        Self {
            offset_low: 0,
            selector: 0,
            ist: 0,
            type_attributes: 0,
            offset_middle: 0,
            offset_high: 0,
            reserved: 0,
        }
    }
}

unsafe fn get_current_cs() -> u16 {
    let cs: u16;
    unsafe {
        core::arch::asm!(
            "mov {:x}, cs",
            out(reg) cs,
            options(nomem, nostack, preserves_flags)
        );
    }
    cs
}

unsafe fn lidt(idt: IdtPointer) {
    unsafe {
        core::arch::asm!(
            "lidt [{}]",
            in(reg) &idt,
            options(readonly, nostack, preserves_flags)
        );
    }
}
