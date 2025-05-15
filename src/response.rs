use crate::host::{handler, memory::BUFFER};

static TYPE: i32 = 1;
pub struct Response {}

impl Response {
    pub fn status_code(&self) -> i32 {
        handler::status_code()
    }
    pub fn set_status_code(&self, code: i32) {
        handler::set_status_code(code);
    }
    pub fn header_names(&self) -> Vec<Vec<u8>> {
        handler::header_names(&BUFFER, TYPE)
    }
    pub fn header_values(&self, name: &[u8]) -> Vec<Vec<u8>> {
        handler::header_values(&BUFFER, TYPE, name)
    }
    pub fn set_header(&self, name: &[u8], value: &[u8]) {
        handler::set_header(TYPE, name, value);
    }
    pub fn add_header_value(&self, name: &[u8], value: &[u8]) {
        handler::add_header_value(TYPE, name, value);
    }
    pub fn remove_header(&self, name: &[u8]) {
        handler::remove_header(TYPE, name);
    }
    pub fn body(&self) -> Option<Vec<u8>> {
        handler::body(&BUFFER, TYPE)
    }
    pub fn write_body(&self, body: &str) {
        handler::write_body(TYPE, body);
    }
}
