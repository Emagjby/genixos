pub mod handlers;
pub mod idt;

use crate::arch::x86_64::interrupts::idt::InterruptDescriptorTable;
use core::cell::UnsafeCell;

static IDT_STORAGE: IdtStorage = IdtStorage::new();

pub fn load() {
    unsafe {
        IDT_STORAGE.initialize();
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
