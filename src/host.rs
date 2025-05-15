use memory::BUFFER;

pub(crate) mod handler;
pub(crate) mod memory;

pub enum Feature {
    BufferRequest = 1,
    BufferResponse = 2,
    Trailers = 4,
}

pub struct Host {}

pub fn get_config() -> Option<String> {
    handler::get_config(&BUFFER)
}

pub fn enable_feature(feature: Feature) -> i32 {
    handler::enable_feature(feature as i32)
}
