use crate::host::{
    handler::{self, method},
    memory::BUFFER,
};
static TYPE: i32 = 0;
pub struct Request {}

impl Request {
    pub fn source_addr(&self) -> Option<Vec<u8>> {
        handler::source_addr(&BUFFER)
    }
    pub fn version(&self) -> Option<Vec<u8>> {
        handler::version(&BUFFER)
    }
    pub fn method(&self) -> Option<Vec<u8>> {
        method(&BUFFER)
    }
    pub fn set_method(&self, method: &[u8]) {
        handler::set_method(method);
    }
    pub fn uri(&self) -> Option<Vec<u8>> {
        handler::uri(&BUFFER)
    }
    pub fn set_uri(&self, uri: &[u8]) {
        handler::set_uri(uri);
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
