use core::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::atomic::{AtomicUsize, Ordering},
};

pub struct SpscBuffer<T, const N: usize> {
    buffer: [UnsafeCell<MaybeUninit<T>>; N],
    head: CachePaddedAtomic,
    tail: CachePaddedAtomic,
}

#[repr(align(64))]
struct CachePaddedAtomic(AtomicUsize);

pub struct Producer<'a, T, const N: usize> {
    shared: &'a SpscBuffer<T, N>,
}

pub struct Consumer<'a, T, const N: usize> {
    shared: &'a SpscBuffer<T, N>,
}

unsafe impl<'a, T: Send, const N: usize> Send for Producer<'a, T, N> {}
unsafe impl<'a, T: Send, const N: usize> Send for Consumer<'a, T, N> {}

#[allow(dead_code)]
impl<T, const N: usize> SpscBuffer<T, N> {
    pub const fn new() -> Self {
        assert!(
            N >= 2 && N.is_power_of_two(),
            "Queue size must be a power of two"
        );

        let buffer =
            unsafe { MaybeUninit::<[UnsafeCell<MaybeUninit<T>>; N]>::uninit().assume_init() };

        Self {
            buffer,
            head: CachePaddedAtomic(AtomicUsize::new(0)),
            tail: CachePaddedAtomic(AtomicUsize::new(0)),
        }
    }

    pub const fn split(&mut self) -> (Producer<'_, T, N>, Consumer<'_, T, N>) {
        (Producer { shared: self }, Consumer { shared: self })
    }
}

impl<'a, T, const N: usize> Producer<'a, T, N> {
    pub fn push(&self, value: T) -> Result<(), T> {
        let head = self.shared.head.0.load(Ordering::Relaxed);
        let tail = self.shared.tail.0.load(Ordering::Acquire);

        if (head.wrapping_add(1) & (N - 1)) == (tail & (N - 1)) {
            return Err(value);
        }

        let slot_idx = head & (N - 1);
        unsafe {
            let ptr = self.shared.buffer[slot_idx].get();
            ptr.write(MaybeUninit::new(value));
        }

        self.shared
            .head
            .0
            .store(head.wrapping_add(1), Ordering::Release);
        Ok(())
    }
}

impl<'a, T, const N: usize> Consumer<'a, T, N> {
    pub fn pop(&self) -> Option<T> {
        let tail = self.shared.tail.0.load(Ordering::Relaxed);
        let head = self.shared.head.0.load(Ordering::Acquire);

        if tail == head {
            return None;
        }

        let slot_idx = tail & (N - 1);
        let value = unsafe {
            let ptr = self.shared.buffer[slot_idx].get();
            ptr.read().assume_init()
        };

        self.shared
            .tail
            .0
            .store(tail.wrapping_add(1), Ordering::Release);
        Some(value)
    }
}

impl<T, const N: usize> Drop for SpscBuffer<T, N> {
    fn drop(&mut self) {
        let mut tail = self.tail.0.load(Ordering::Relaxed);
        let head = self.head.0.load(Ordering::Acquire);

        while tail != head {
            let slot_idx = tail & (N - 1);
            unsafe {
                let ptr = self.buffer[slot_idx].get();
                ptr.read().assume_init_drop();
            }
            tail = tail.wrapping_add(1);
        }
    }
}
