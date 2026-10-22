use crate::arch::x86_64::interrupts::tss::TaskStateSegment;

pub const KERNEL_CODE_SELECTOR: u16 = 0x08;
pub const KERNEL_DATA_SELECTOR: u16 = 0x10;
pub const TSS_SELECTOR: u16 = 0x18;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct GdtEntry {
    pub limit: u16,
    pub low_base: u16,
    pub middle_base: u8,
    pub access: u8,
    pub flags: u8,
    pub top_base: u8,
}

#[repr(C, packed)]
struct GdtPointer {
    limit: u16,
    base: u64,
}

impl GdtEntry {
    pub const fn new(
        limit: u16,
        low_base: u16,
        middle_base: u8,
        access: u8,
        flags: u8,
        top_base: u8,
    ) -> Self {
        Self {
            limit,
            low_base,
            middle_base,
            access,
            flags,
            top_base,
        }
    }

    pub const fn empty() -> Self {
        Self {
            limit: 0,
            low_base: 0,
            middle_base: 0,
            access: 0,
            flags: 0,
            top_base: 0,
        }
    }
}

#[repr(C)]
#[repr(align(16))]
pub struct GlobalDescriptorTable {
    entries: [GdtEntry; 5],
}

impl GlobalDescriptorTable {
    pub const fn new() -> Self {
        const NULL: GdtEntry = GdtEntry::empty();

        Self { entries: [NULL; 5] }
    }

    pub fn add_code_descriptor(&mut self) {
        self.set_gdt_gate(1, 0, 0xFFFFFFFF, 0x9A, 0xAF);
    }

    pub fn add_data_descriptor(&mut self) {
        self.set_gdt_gate(2, 0, 0xFFFFFFFF, 0x92, 0xCF);
    }

    pub fn add_tss_descriptor(&mut self, idx: usize, tss: *const TaskStateSegment) {
        let base: u64 = tss as u64;
        let limit: u64 = (size_of::<TaskStateSegment>() - 1) as u64;

        self.set_tss_gate(idx, base, limit);
    }

    pub fn load(&'static self) {
        let pointer = GdtPointer {
            limit: (size_of::<[GdtEntry; 5]>() - 1) as u16,
            base: self.entries.as_ptr() as u64,
        };

        unsafe { lgdt(pointer) }
    }

    fn set_gdt_gate(&mut self, idx: usize, base: u32, limit: u32, access: u8, gran: u8) {
        self.entries[idx].low_base = (base & 0xFFFF) as u16;
        self.entries[idx].middle_base = ((base >> 16) & 0xFF) as u8;
        self.entries[idx].top_base = ((base >> 24) & 0xFF) as u8;

        self.entries[idx].limit = (limit & 0xFFFF) as u16;
        self.entries[idx].flags = ((limit >> 16) & 0x0F) as u8;
        self.entries[idx].flags |= gran & 0xF0;

        self.entries[idx].access = access;
    }

    fn set_tss_gate(&mut self, idx: usize, base: u64, limit: u64) {
        self.entries[idx].limit = (limit & 0xFFFF) as u16;

        self.entries[idx].low_base = (base & 0xFFFF) as u16;
        self.entries[idx].middle_base = ((base >> 16) & 0xFF) as u8;
        self.entries[idx].top_base = ((base >> 24) & 0xFF) as u8;

        self.entries[idx].access = 0x89;
        self.entries[idx].flags = ((limit >> 16) & 0x0F) as u8;

        // second byte slot
        self.entries[idx + 1].limit = ((base >> 32) & 0xFFFF) as u16;

        self.entries[idx + 1].low_base = ((base >> 48) & 0xFFFF) as u16;
        self.entries[idx + 1].middle_base = 0;
        self.entries[idx + 1].top_base = 0;

        self.entries[idx + 1].access = 0;
        self.entries[idx + 1].flags = 0;
    }
}

unsafe fn lgdt(gdt: GdtPointer) {
    unsafe {
        core::arch::asm!(
            "lgdt [{}]",
            in(reg) &gdt,
            options(readonly, nostack, preserves_flags)
        )
    }
}

pub unsafe fn ltr(selector: u16) {
    unsafe {
        core::arch::asm!(
            "ltr {0:x}",
            in(reg) selector,
            options(nostack, preserves_flags)
        )
    }
}

pub unsafe fn reload_cs(selector: u16) {
    unsafe {
        core::arch::asm!(
            "push {selector:r}",
            "lea {tmp}, [rip + 2f]",
            "push {tmp}",
            "retfq",
            "2:",
            selector = in(reg) selector as u64,
            tmp = lateout(reg) _,
            options(preserves_flags)
        )
    }
}

pub unsafe fn reload_data_segments(selector: u16) {
    unsafe {
        core::arch::asm!(
            "mov ds, {kds:x}",
            "mov es, {kds:x}",
            "mov ss, {kds:x}",
            kds = in(reg) selector,
            options(nostack, preserves_flags)
        )
    }
}
