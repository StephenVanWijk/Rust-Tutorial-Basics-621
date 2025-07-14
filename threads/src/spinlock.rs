use std::cell::UnsafeCell;
use std::hint;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};

/// A simple spinlock implementation.
pub struct SpinLock<T> {
    /// `AtomicBool` is used to track the lock's state (locked/unlocked).
    locked: AtomicBool,
    /// `UnsafeCell` provides interior mutability. It allows us to get a mutable
    /// reference to the data even when the `SpinLock` is behind an immutable reference.
    /// This is safe because we ensure exclusive access via the `locked` atomic.
    value: UnsafeCell<T>,
}

/// A guard that provides access to the locked data.
/// When the guard is dropped, the lock is automatically released.
pub struct SpinLockGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

// These `unsafe` impls are necessary to tell the compiler that `SpinLock<T>`
// is thread-safe if `T` is `Send`. The `UnsafeCell` makes the compiler
// conservative, but we know our implementation is sound.
unsafe impl<T: Send> Send for SpinLock<T> {}
unsafe impl<T: Send> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
    /// Creates a new `SpinLock` protecting the given data.
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    /// Acquires the lock, spinning until it is available.
    /// Returns a `SpinLockGuard` which allows access to the data.
    pub fn lock(&self) -> SpinLockGuard<T> {
        // This is the "spin" part of the spinlock.
        // We loop continuously until we successfully acquire the lock.
        while self
            .locked
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // `compare_exchange_weak` is used for performance in loops.
            // If it fails, it means the lock is currently held.
            // We use `std::hint::spin_loop()` to inform the processor that we are
            // in a busy-wait loop, which can improve performance.
            hint::spin_loop();
        }

        // Once we acquire the lock, we return a guard.
        SpinLockGuard { lock: self }
    }
}

/// Implements the `Drop` trait to automatically release the lock when the
/// `SpinLockGuard` goes out of scope. This is a crucial part of the lock guard pattern.
impl<T> Drop for SpinLockGuard<'_, T> {
    fn drop(&mut self) {
        // The `Release` ordering ensures that any writes to the protected data
        // "happen-before" the next thread acquires the lock, making our changes
        // visible to it and preventing data races.
        self.lock.locked.store(false, Ordering::Release);
    }
}

/// Implements `Deref` to allow the `SpinLockGuard` to be treated like a reference
/// to the protected data.
impl<T> Deref for SpinLockGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // Safety: The existence of the guard proves we have exclusive access.
        // The `Acquire` memory ordering in `lock()` ensures we see all previous writes.
        unsafe { &*self.lock.value.get() }
    }
}

/// Implements `DerefMut` to allow the `SpinLockGuard` to be treated like a mutable
/// reference to the protected data.
impl<T> DerefMut for SpinLockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: The existence of the guard proves we have exclusive access.
        unsafe { &mut *self.lock.value.get() }
    }
}