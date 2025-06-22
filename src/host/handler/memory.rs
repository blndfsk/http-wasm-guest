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
    pub fn size(&self) -> i32 {
        self.data.len() as i32
    }
}

pub(crate) fn buffer() -> &'static LazyLock<Buffer> {
    &BUFFER
}
