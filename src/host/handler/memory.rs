use std::sync::LazyLock;

static BUFFER: LazyLock<Buffer> = LazyLock::new(Buffer::new);
const SIZE: usize = 2048;

pub(crate) struct Buffer {
    pub data: [u8; SIZE],
}
impl Buffer {
    fn new() -> Buffer {
        Self { data: [0u8; SIZE] }
    }
    #[inline]
    pub fn len(&self) -> i32 {
        self.data.len() as i32
    }
    #[cfg(test)]
    pub fn from_vec(data: &[u8]) -> Buffer {
        let mut buffer = [0; SIZE];
        buffer[..data.len()].clone_from_slice(data);
        Self { data: buffer }
    }
}

pub(crate) fn buffer() -> &'static LazyLock<Buffer> {
    &BUFFER
}
