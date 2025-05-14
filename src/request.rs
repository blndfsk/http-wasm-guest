use http::{
    HeaderName, HeaderValue, Method, Uri, Version,
    header::{InvalidHeaderName, InvalidHeaderValue},
};
use std::{net::IpAddr, str::FromStr};

use crate::{Type, host::handler};

pub struct Request {}

impl Request {
    pub fn source_addr(&self) -> Option<IpAddr> {
        handler::source_addr().and_then(|str| IpAddr::from_str(str.as_str()).ok())
    }

    pub fn method(&self) -> Option<Method> {
        handler::method().and_then(|src| Method::from_bytes(&src).ok())
    }

    pub fn set_method(&self, method: &Method) {
        let method = method.as_str();
        handler::set_method(method);
    }

    pub fn uri(&self) -> Option<Uri> {
        handler::uri().and_then(|str| Uri::from_str(str.as_str()).ok())
    }

    pub fn set_uri(&self, uri: Uri) {
        let uri = uri.to_string();
        handler::set_uri(uri.as_str());
    }

    pub fn version(&self) -> Version {
        handler::version()
            .map(|src| match src.as_slice() {
                b"HTTP/0.9" => Version::HTTP_09,
                b"HTTP/1.0" => Version::HTTP_10,
                b"HTTP/1.1" => Version::HTTP_11,
                b"HTTP/2.0" => Version::HTTP_2,
                b"HTTP/3.0" => Version::HTTP_3,
                _ => Version::default(),
            })
            .unwrap()
    }

    pub fn header_names(&self) -> Vec<Result<HeaderName, InvalidHeaderName>> {
        handler::header_names(Type::Request as u32)
            .iter()
            .map(|src| HeaderName::from_bytes(src))
            .collect()
    }

    pub fn header_values(&self, name: &HeaderName) -> Vec<Result<HeaderValue, InvalidHeaderValue>> {
        handler::header_values(Type::Request as u32, name.as_str())
            .iter()
            .map(|src| HeaderValue::from_bytes(src))
            .collect()
    }
    pub fn set_header(&self, name: &HeaderName, value: &HeaderValue) {
        handler::set_header(Type::Request as u32, name.as_str(), value.as_bytes());
    }
    pub fn add_header_value(&self, name: &HeaderName, value: &HeaderValue) {
        handler::add_header_value(Type::Request as u32, name.as_str(), value.as_bytes());
    }
    pub fn remove_header(&self, name: &HeaderName) {
        handler::remove_header(Type::Request as u32, name.as_str());
    }
    pub fn body(&self) -> Option<String> {
        handler::body(Type::Request as u32)
    }
    pub fn write_body(&self, body: &str) {
        handler::write_body(Type::Request as u32, body);
    }
}
