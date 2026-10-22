pub mod gdt;
pub mod handlers;
pub mod idt;
pub mod tss;

use crate::arch::x86_64::interrupts::{
    gdt::GlobalDescriptorTable,
    idt::InterruptDescriptorTable,
    tss::{DoubleFaultStack, TaskStateSegment},
};
use core::cell::UnsafeCell;

static DOUBLE_FAULT_STACK: DfsStorage = DfsStorage::new();

static IDT_STORAGE: IdtStorage = IdtStorage::new();
static GDT_STORAGE: GdtStorage = GdtStorage::new();
static TSS_STORAGE: TssStorage = TssStorage::new();

pub fn load() {
    unsafe {
        TSS_STORAGE.initialize();
        GDT_STORAGE.initialize();
        IDT_STORAGE.initialize();
    }
}

pub struct GdtStorage {
    gdt: UnsafeCell<GlobalDescriptorTable>,
}

impl GdtStorage {
    pub const fn new() -> Self {
        Self {
            gdt: UnsafeCell::new(GlobalDescriptorTable::new()),
        }
    }

    pub unsafe fn initialize(&self) {
        use crate::arch::x86_64::interrupts::gdt;

        unsafe {
            let gdt_pointer = self.gdt.get();

            let gdt = &mut *gdt_pointer;

            gdt.add_code_descriptor();
            gdt.add_data_descriptor();

            gdt.add_tss_descriptor(3, TSS_STORAGE.address());

            gdt.load();

            gdt::reload_cs(gdt::KERNEL_CODE_SELECTOR);
            gdt::reload_data_segments(gdt::KERNEL_DATA_SELECTOR);
            gdt::ltr(gdt::TSS_SELECTOR);
        }
    }
}

pub struct DfsStorage {
    dfs: UnsafeCell<DoubleFaultStack>,
}

impl DfsStorage {
    pub const fn new() -> Self {
        Self {
            dfs: UnsafeCell::new(DoubleFaultStack::new()),
        }
    }

    pub fn stack_top(&self) -> *mut u8 {
        (self.dfs.get() as *mut u8).wrapping_add(size_of::<DoubleFaultStack>())
    }
}

pub struct TssStorage {
    tss: UnsafeCell<TaskStateSegment>,
}

impl TssStorage {
    pub const fn new() -> Self {
        Self {
            tss: UnsafeCell::new(TaskStateSegment::new()),
        }
    }

    pub fn address(&self) -> *const TaskStateSegment {
        self.tss.get()
    }

    pub unsafe fn initialize(&self) {
        unsafe {
            let tss_pointer = self.tss.get();

            let tss = &mut *tss_pointer;

            let dfs_top = DOUBLE_FAULT_STACK.stack_top();
            tss.set_ist(1, dfs_top);
        }
    }
}

pub struct IdtStorage {
    idt: UnsafeCell<InterruptDescriptorTable>,
}

impl IdtStorage {
    pub const fn new() -> Self {
        Self {
            idt: UnsafeCell::new(InterruptDescriptorTable::new()),
        }
    }

    pub unsafe fn initialize(&self) {
        unsafe {
            let idt_pointer = self.idt.get();

            let idt = &mut *idt_pointer;

            idt.set_handler(3, handlers::breakpoint_handler as *const () as u64);

            idt.set_handler(8, handlers::double_fault_handler as *const () as u64);
            idt.set_ist(8, 1);

            idt.load();
        }
    }
}

// SAFETY:
// Only the bootstrap CPU initializes this IDT.
// Initialization happens exactly once.
// No mutation occurs after LIDT.
// No concurrent access is permitted during intialization.
unsafe impl Sync for IdtStorage {}

unsafe impl Sync for GdtStorage {}

unsafe impl Sync for TssStorage {}

unsafe impl Sync for DfsStorage {}
