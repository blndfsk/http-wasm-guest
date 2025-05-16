use memory::buffer;

pub(crate) mod handler;
pub(crate) mod memory;

pub enum Feature {
    BufferRequest = 1,
    BufferResponse = 2,
    Trailers = 4,
}

pub struct Host {}

pub fn get_config() -> Option<Vec<u8>> {
    handler::get_config(buffer())
}

pub fn enable_feature(feature: Feature) -> i32 {
    handler::enable_feature(feature as i32)
}
