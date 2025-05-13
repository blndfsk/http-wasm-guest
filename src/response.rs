use http::{
    HeaderName, HeaderValue, StatusCode,
    header::{InvalidHeaderName, InvalidHeaderValue},
};

use crate::{Type, host::handler};

pub struct Response {}

impl Response {
    pub fn status_code(&self) -> Option<StatusCode> {
        StatusCode::from_u16(handler::status_code() as u16).ok()
    }
    pub fn set_status_code(&self, code: &StatusCode) {
        handler::set_status_code(code.as_u16() as i32);
    }

    pub fn header_names(&self) -> Vec<Result<HeaderName, InvalidHeaderName>> {
        handler::header_names(Type::Response as u32)
            .iter()
            .map(|src| HeaderName::from_bytes(src))
            .collect()
    }

    pub fn header_values(&self, name: &HeaderName) -> Vec<Result<HeaderValue, InvalidHeaderValue>> {
        handler::header_values(Type::Response as u32, name.as_str())
            .iter()
            .map(|src| HeaderValue::from_bytes(src))
            .collect()
    }
    pub fn set_header(&self, name: &HeaderName, value: &HeaderValue) {
        handler::set_header(Type::Response as u32, name.as_str(), value.as_bytes());
    }
    pub fn add_header_value(&self, name: &HeaderName, value: &HeaderValue) {
        handler::add_header_value(Type::Response as u32, name.as_str(), value.as_bytes());
    }
    pub fn remove_header(&self, name: &HeaderName) {
        handler::remove_header(Type::Response as u32, name.as_str());
    }
    pub fn body(&self) -> Option<String> {
        handler::body(Type::Response as u32)
    }
    pub fn write_body(&self, body: &str) {
        handler::write_body(Type::Response as u32, body);
    }
}
