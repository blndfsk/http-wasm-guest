use std::sync::{LazyLock, RwLock, RwLockWriteGuard};

static BUFFER: RwLock<LazyLock<Buffer>> = RwLock::new(LazyLock::new(Buffer::new));
const SIZE: usize = 2048;

pub(crate) struct Buffer {
    data: [u8; SIZE],
}
impl Buffer {
    fn new() -> Buffer {
        Self { data: [0u8; SIZE] }
    }
    #[inline]
    pub fn len(&self) -> i32 {
        self.data.len() as i32
    }
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.data.as_mut_ptr()
    }
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }
    pub fn as_subslice(&self, size: i32) -> &[u8] {
        &self.data[0..size as usize]
    }
    pub fn to_boxed_slice(&self, size: i32) -> Box<[u8]> {
        self.as_subslice(size).to_vec().into_boxed_slice()
    }
    #[cfg(test)]
    pub fn from_vec(data: &[u8]) -> Buffer {
        let mut buffer = [0; SIZE];
        buffer[..data.len()].clone_from_slice(data);
        Self { data: buffer }
    }
}

pub(crate) fn buffer() -> RwLockWriteGuard<'static, LazyLock<Buffer>> {
    match BUFFER.write() {
        Ok(buf) => buf,
        Err(err) => {
            log::error!("{}", err);
            BUFFER.clear_poison();
            buffer()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_slice() {
        let c = b"test";
        let buf = Buffer::from_vec(c);
        let r = buf.as_subslice(c.len() as i32);
        assert_eq!(c, r);
    }
    #[test]
    fn test_as_slice_empty() {
        let c = b"";
        let buf = Buffer::from_vec(c);
        let r = buf.as_subslice(c.len() as i32);
        assert!(r.is_empty());
    }
}
