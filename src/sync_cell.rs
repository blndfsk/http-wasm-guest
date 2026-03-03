//! A thin wrapper around [`UnsafeCell`] that implements [`Sync`].
//!
//! This is intended **exclusively** for single-threaded WebAssembly guests
//! where no concurrent access is possible. It allows storing mutable state
//! in `static` items without the overhead of locks or atomic synchronisation.

use std::cell::UnsafeCell;

/// An [`UnsafeCell`] wrapper that is [`Sync`], enabling use in `static` items.
///
/// # Safety
///
/// This type is **only** sound when used in a single-threaded context (e.g. a
/// WASM guest). The caller is responsible for ensuring that no aliasing mutable
/// references exist when accessing the inner value.
pub(crate) struct SyncCell<T>(UnsafeCell<T>);

impl<T> SyncCell<T> {
    /// Create a new `SyncCell` with the given value.
    pub(crate) const fn new(value: T) -> Self {
        Self(UnsafeCell::new(value))
    }

    /// Return a raw pointer to the inner value.
    ///
    /// # Safety
    ///
    /// The caller must ensure exclusive access before dereferencing the pointer
    /// mutably. In a single-threaded WASM guest this is always the case.
    #[inline]
    pub(crate) const fn get(&self) -> *mut T {
        self.0.get()
    }
}

// SAFETY: WASM guests are single-threaded; no concurrent access is possible.
unsafe impl<T> Sync for SyncCell<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sync_cell_basic_usage() {
        let cell = SyncCell::new(42);
        // SAFETY: single-threaded, no aliasing
        let value = unsafe { &mut *cell.get() };
        assert_eq!(*value, 42);
        *value = 99;
        assert_eq!(*value, 99);
    }
}
