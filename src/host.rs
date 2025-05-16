use memory::buffer;

pub(crate) mod handler;
pub(crate) mod memory;

#[allow(non_upper_case_globals, non_snake_case)]
pub mod Feature {
    pub const BufferRequest: crate::Feature = crate::Feature(1);
    pub const BufferResponse: crate::Feature = crate::Feature(2);
    pub const Trailers: crate::Feature = crate::Feature(4);
}
pub struct Host {}

pub fn get_config() -> Option<Vec<u8>> {
    handler::get_config(buffer())
}

pub fn enable_feature(feature: crate::Feature) -> i32 {
    handler::enable_feature(feature.0)
}
