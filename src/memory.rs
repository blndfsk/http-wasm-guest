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
        &self.data[..size.min(SIZE)]
    }

    /// returns a copy of the contents as an owned type
    pub(crate) fn to_boxed_slice(&self, size: usize) -> Box<[u8]> {
        Box::from(self.as_subslice(size))
    }

    #[cfg(test)]
    pub(super) fn from_slice(data: &[u8]) -> Buffer {
        let mut buffer = [0; SIZE];
        let len = data.len().min(SIZE);
        buffer[..len].clone_from_slice(&data[..len]);
        Self { data: buffer }
    }
}

static BUFFER: SyncCell<Buffer> = SyncCell::new(Buffer::new());

/// Provides scoped, exclusive access to the global buffer.
///
/// The closure-based API ensures that only one `&mut Buffer` reference exists
/// at a time, preventing the aliasing UB that a bare `&'static mut` return
/// would allow.
///
/// # Safety
///
/// Sound only in a single-threaded context (WASM guest). The closure must not
/// re-enter `with_buffer`.
pub(crate) fn with_buffer<R>(f: impl FnOnce(&mut Buffer) -> R) -> R {
    // SAFETY: WASM guest is single-threaded; the closure scope guarantees
    // that no second &mut reference can coexist.
    let buf = unsafe { &mut *BUFFER.get() };
    f(buf)
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
    fn test_as_subslice() {
        let c = b"test";
        let buf = Buffer::from_slice(c);
        let r = buf.as_subslice(c.len());
        assert_eq!(c, r);
    }

    #[test]
    fn test_as_subslice_empty() {
        let c = b"";
        let buf = Buffer::from_slice(c);
        let r = buf.as_subslice(c.len());
        assert!(r.is_empty());
    }

    #[test]
    fn test_as_subslice_full() {
        let buf = Buffer::new();
        let r = buf.as_subslice(SIZE);
        assert_eq!(r.len(), SIZE);
    }

    #[test]
    fn test_as_subslice_clamps_oversize() {
        let buf = Buffer::new();
        let r = buf.as_subslice(SIZE + 1000);
        assert_eq!(r.len(), SIZE);
    }

    #[test]
    fn test_to_boxed_slice() {
        let c = b"hello";
        let buf = Buffer::from_slice(c);
        let boxed = buf.to_boxed_slice(c.len());
        assert_eq!(boxed.as_ref(), c);
    }

    #[test]
    fn test_to_boxed_slice_empty() {
        let buf = Buffer::new();
        let boxed = buf.to_boxed_slice(0);
        assert!(boxed.is_empty());
    }

    #[test]
    fn test_to_boxed_slice_clamps_oversize() {
        let buf = Buffer::new();
        let boxed = buf.to_boxed_slice(SIZE + 500);
        assert_eq!(boxed.len(), SIZE);
    }

    #[test]
    fn test_as_mut_ptr() {
        let mut buf = Buffer::from_slice(b"abc");
        let ptr = buf.as_mut_ptr();
        unsafe { *ptr = b'X' };
        assert_eq!(buf.as_subslice(3), b"Xbc");
    }

    #[test]
    fn test_as_mut_slice() {
        let mut buf = Buffer::from_slice(b"abc");
        let slice = buf.as_mut_slice();
        slice[0] = b'Z';
        assert_eq!(buf.as_subslice(3), b"Zbc");
    }

    #[test]
    fn test_from_slice_clamps_oversize() {
        let large = vec![0xFFu8; SIZE + 500];
        let buf = Buffer::from_slice(&large);
        // All SIZE bytes should be 0xFF
        assert!(buf.as_slice().iter().all(|&b| b == 0xFF));
    }

    #[test]
    fn test_with_buffer_returns_value() {
        let result = with_buffer(|buf| buf.capacity());
        assert_eq!(result, SIZE);
    }

    #[test]
    fn test_with_buffer_mutation() {
        with_buffer(|buf| {
            buf.as_mut_slice()[0] = 42;
        });
        with_buffer(|buf| {
            assert_eq!(buf.as_slice()[0], 42);
            // Reset for other tests
            buf.as_mut_slice()[0] = 0;
        });
    }
}
