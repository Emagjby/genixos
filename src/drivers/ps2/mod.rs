use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
};

use crate::{
    collections::spsc::{Consumer, Producer, SpscBuffer},
    drivers::ps2::keyboard::Ps2Keyboard,
};

pub mod keyboard;

static KEYBOARD: Ps2Keyboard = Ps2Keyboard::new();
static SCANCODE_QUEUE: QueueStorage = QueueStorage::new();
static PRODUCER: ProducerStorage = ProducerStorage::new();

pub fn receive_input() {
    if let Some(byte) = KEYBOARD.read_scancode() {
        let _ = PRODUCER.push(byte);
    }
}

pub fn initialize() -> Consumer<'static, u8, 64> {
    let claimed = SCANCODE_QUEUE.initialized.compare_exchange(
        false,
        true,
        Ordering::AcqRel,
        Ordering::Acquire,
    );
    assert!(claimed.is_ok(), "Scancode queue is already initialized");

    let (producer, consumer) = SCANCODE_QUEUE.split();
    PRODUCER.initialize(producer);

    consumer
}

struct ProducerStorage {
    producer: UnsafeCell<Option<Producer<'static, u8, 64>>>,
}

impl ProducerStorage {
    pub const fn new() -> Self {
        Self {
            producer: UnsafeCell::new(None),
        }
    }

    pub fn initialize(&self, producer: Producer<'static, u8, 64>) {
        unsafe {
            *self.producer.get() = Some(producer);
        }
    }

    pub fn push(&self, value: u8) -> Result<(), u8> {
        unsafe {
            let ptr = self.producer.get();
            let inner = &*ptr;

            if let Some(prod) = inner {
                return prod.push(value);
            }
            Err(value)
        }
    }
}

struct QueueStorage {
    buffer: UnsafeCell<SpscBuffer<u8, 64>>,
    initialized: AtomicBool,
}

impl QueueStorage {
    pub const fn new() -> Self {
        Self {
            buffer: UnsafeCell::new(SpscBuffer::<u8, 64>::new()),
            initialized: AtomicBool::new(false),
        }
    }

    fn split(&self) -> (Producer<'_, u8, 64>, Consumer<'_, u8, 64>) {
        unsafe {
            let ptr = self.buffer.get();
            let buf = &mut *ptr;
            buf.split()
        }
    }
}

unsafe impl Sync for QueueStorage {}
unsafe impl Sync for ProducerStorage {}
