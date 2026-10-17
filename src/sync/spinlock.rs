use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

/// A non-reentrant spinlock protecting shared data.
///
/// Blocking acquisition spins until the lock becomes available.
/// Recursive acquisition causes deadlock.
/// This implementation does not disable interrupts.
/// Caller must avoid acquiring it from interrupt contexts that
/// can interrupt an exisiting holder.
pub struct SpinLock<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

pub struct SpinLockGuard<'a, T> {
    inner: &'a SpinLock<T>,
}

// SAFETY:
// The atomic lock ensures that only one guard can access the
// protected value at a time. Acquire/Release ordering synchronizes
// access between execution contexts. T must be Send.
unsafe impl<T: Send> Sync for SpinLock<T> {}

// SAFETY:
// Ownership of the lock can be transferred between execution
// contexts when T is Send. Access to the protected value remains
// controlled by the lock.
unsafe impl<T: Send> Send for SpinLock<T> {}

impl<T> SpinLock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> SpinLockGuard<'_, T> {
        loop {
            if let Some(guard) = self.try_lock() {
                break guard;
            }

            core::hint::spin_loop()
        }
    }

    pub fn try_lock(&self) -> Option<SpinLockGuard<'_, T>> {
        if self
            .locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            Some(SpinLockGuard { inner: self })
        } else {
            None
        }
    }
}

impl<'a, T> Deref for SpinLockGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        // SAFETY:
        // A SpinLockGuard can only be created after acquiring the lock.
        // The guard's lifetime keeps the lock held while this reference
        // exists, preventing access through another guard.
        unsafe { &*self.inner.data.get() }
    }
}

impl<'a, T> DerefMut for SpinLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY:
        // The guard holds exclusive access to the protected value.
        // No other guard can exist while this lock is held, allowing
        // a mutable reference to be created.
        unsafe { &mut *self.inner.data.get() }
    }
}

impl<'a, T> Drop for SpinLockGuard<'a, T> {
    fn drop(&mut self) {
        self.inner.locked.store(false, Ordering::Release);
    }
}
