pub(crate) mod handler;
pub(crate) mod memory;

#[allow(non_snake_case, non_upper_case_globals)]
pub mod Feature {
    pub const BufferRequest: u32 = 1;
    pub const BufferResponse: u32 = 2;
    pub const Trailers: u32 = 4;
}

pub struct Host {}

pub fn get_config() -> Option<String> {
    handler::get_config()
}

pub fn enable_feature(feature: u32) -> i32 {
    handler::enable_feature(feature)
}
