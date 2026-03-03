use crate::sync_cell::SyncCell;

const SIZE: usize = 2048;

pub(crate) struct Buffer {
    data: [u8; SIZE],
}

impl Buffer {
    const fn new() -> Buffer {
        Self { data: [0u8; SIZE] }
    }
    #[inline]
    pub(crate) fn len(&self) -> i32 {
        self.data.len() as i32
    }
    pub(crate) fn as_mut_ptr(&mut self) -> *mut u8 {
        self.data.as_mut_ptr()
    }
    pub(crate) fn as_slice(&self) -> &[u8] {
        &self.data
    }
    pub(crate) fn as_subslice(&self, size: i32) -> &[u8] {
        &self.data[0..size as usize]
    }
    /// returns a copy of the contents as an owned type
    pub(crate) fn to_boxed_slice(&self, size: i32) -> Box<[u8]> {
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
    fn test_new_buffer() {
        let buf = Buffer::new();
        assert_eq!(buf.len(), SIZE as i32);
        assert!(buf.as_slice().iter().all(|&b| b == 0));
    }
    #[test]
    fn test_as_slice() {
        let c = b"test";
        let buf = Buffer::from_slice(c);
        let r = buf.as_subslice(c.len() as i32);
        assert_eq!(c, r);
    }
    #[test]
    fn test_as_slice_empty() {
        let c = b"";
        let buf = Buffer::from_slice(c);
        let r = buf.as_subslice(c.len() as i32);
        assert!(r.is_empty());
    }
}
