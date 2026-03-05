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

const SIZE: usize = 2048;

pub(crate) struct Buffer {
    data: [u8; SIZE],
}

impl Buffer {
    const fn new() -> Buffer {
        Self { data: [0u8; SIZE] }
    }

    #[inline]
    pub(crate) fn capacity(&self) -> usize {
        SIZE
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut u8 {
        self.data.as_mut_ptr()
    }
    pub(crate) fn as_slice(&self) -> &[u8] {
        &self.data
    }
    pub(crate) fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data
    }
    pub(crate) fn as_subslice(&self, size: usize) -> &[u8] {
        &self.data[0..size]
    }

    /// returns a copy of the contents as an owned type
    pub(crate) fn to_boxed_slice(&self, size: usize) -> Box<[u8]> {
        Box::from(self.as_subslice(size))
    }

    #[cfg(test)]
    pub(super) fn from_slice(data: &[u8]) -> Buffer {
        let mut buffer = [0; SIZE];
        buffer[..data.len()].clone_from_slice(data);
        Self { data: buffer }
    }
}

static BUFFER: SyncCell<Buffer> = SyncCell::new(Buffer::new());

/// Returns a mutable reference to the global buffer.
pub(crate) fn buffer() -> &'static mut Buffer {
    // SAFETY: WASM guest is single-threaded.
    unsafe { &mut *BUFFER.get() }
}

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

    #[test]
    fn test_new_buffer() {
        let buf = Buffer::new();
        assert_eq!(buf.capacity(), SIZE);
        assert!(buf.as_slice().iter().all(|&b| b == 0));
    }
    #[test]
    fn test_as_slice() {
        let c = b"test";
        let buf = Buffer::from_slice(c);
        let r = buf.as_subslice(c.len());
        assert_eq!(c, r);
    }
    #[test]
    fn test_as_slice_empty() {
        let c = b"";
        let buf = Buffer::from_slice(c);
        let r = buf.as_subslice(c.len());
        assert!(r.is_empty());
    }
}
