pub mod cpu;
pub mod gdt;
pub mod handlers;
pub mod idt;
pub mod pic;
pub mod pit;
pub mod ticks;
pub mod tss;

use crate::arch::x86_64::interrupts::{
    cpu::{disable_interrupts, enable_interrupts},
    gdt::GlobalDescriptorTable,
    idt::InterruptDescriptorTable,
    pic::ChainedPics,
    pit::Pit,
    ticks::Counter,
    tss::{DoubleFaultStack, TaskStateSegment},
};
use core::cell::UnsafeCell;

static DOUBLE_FAULT_STACK: DfsStorage = DfsStorage::new();

static IDT_STORAGE: IdtStorage = IdtStorage::new();
static GDT_STORAGE: GdtStorage = GdtStorage::new();
static TSS_STORAGE: TssStorage = TssStorage::new();

static PIC_STORAGE: PicStorage = PicStorage::new();
static PIT_STORAGE: PitStorage = PitStorage::new();

static TICK_COUNTER: Counter = Counter::new();

pub fn load() {
    unsafe {
        disable_interrupts();
        TSS_STORAGE.initialize();
        GDT_STORAGE.initialize();
        IDT_STORAGE.initialize();
        PIC_STORAGE.initialize();
        PIT_STORAGE.initialize();
        PIC_STORAGE.unmask_irq(0);
        enable_interrupts();
    }
}

pub struct PitStorage {
    pit: UnsafeCell<Pit>,
}

impl PitStorage {
    pub const fn new() -> Self {
        Self {
            pit: UnsafeCell::new(Pit::new()),
        }
    }

    pub unsafe fn initialize(&self) {
        unsafe {
            let pit_pointer = self.pit.get();

            let pit = &*pit_pointer;

            pit.configure_periodic(100); // Hz
        }
    }
}

pub struct PicStorage {
    pic: UnsafeCell<ChainedPics>,
}

impl PicStorage {
    pub const fn new() -> Self {
        Self {
            pic: UnsafeCell::new(ChainedPics::new()),
        }
    }

    pub unsafe fn initialize(&self) {
        unsafe {
            let pic_pointer = self.pic.get();
            let pic = &*pic_pointer;

            pic.initialize();
        }
    }

    pub fn send_eoi(&self, irq: u8) {
        unsafe {
            let pic_pointer = self.pic.get();
            let pic = &*pic_pointer;

            pic.send_eoi(irq);
        }
    }

    pub fn unmask_irq(&self, irq: u8) {
        unsafe {
            let pic_pointer = self.pic.get();
            let pic = &*pic_pointer;

            pic.unmask_irq(irq)
        }
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

            idt.set_handler(0x20, handlers::timer_interrupt_handler as *const () as u64);

            idt.load();
        }
    }
}

pub fn pic_send_eoi(irq: u8) {
    PIC_STORAGE.send_eoi(irq);
}

pub fn tick_increment() {
    TICK_COUNTER.increment();
}

pub fn get_current_tick() -> u64 {
    TICK_COUNTER.read()
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

unsafe impl Sync for PicStorage {}

unsafe impl Sync for PitStorage {}
