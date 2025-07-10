use std::{fmt::Display, ops::Deref, string::FromUtf8Error};

pub mod feature;
mod handler;
pub mod log;

pub fn get_config() -> Result<String, FromUtf8Error> {
    String::from_utf8(handler::get_config())
}

static KIND_REQ: i32 = 0;
static KIND_RES: i32 = 1;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Bytes(Box<[u8]>);
impl Bytes {
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0).unwrap_or_default()
    }
}
impl Deref for Bytes {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}
impl Display for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl From<&str> for Bytes {
    fn from(value: &str) -> Self {
        Self(value.as_bytes().to_vec().into_boxed_slice())
    }
}
impl From<&[u8]> for Bytes {
    fn from(value: &[u8]) -> Self {
        Self(value.to_vec().into_boxed_slice())
    }
}

pub struct Header {
    kind: i32,
}
impl Header {
    pub fn names(&self) -> Vec<Bytes> {
        handler::header_names(self.kind)
            .iter()
            .map(|h| Bytes(h.clone()))
            .collect()
    }
    pub fn values(&self, name: &Bytes) -> Vec<Bytes> {
        handler::header_values(self.kind, name)
            .iter()
            .map(|h| Bytes(h.clone()))
            .collect()
    }
    pub fn set(&self, name: &Bytes, value: &Bytes) {
        handler::set_header(self.kind, name, value);
    }
    pub fn add(&self, name: &Bytes, value: &Bytes) {
        handler::add_header_value(self.kind, name, value);
    }
    pub fn remove(&self, name: &Bytes) {
        handler::remove_header(self.kind, name);
    }
}
pub struct Body {
    kind: i32,
}
impl Body {
    pub fn read(&self) -> Bytes {
        Bytes(handler::body(self.kind))
    }
    pub fn write(&self, body: &Bytes) {
        handler::write_body(self.kind, body);
    }
}

pub struct Request {
    header: Header,
    body: Body,
}

impl Default for Request {
    fn default() -> Self {
        Self::new()
    }
}

impl Request {
    pub fn new() -> Self {
        Self {
            header: Header { kind: KIND_REQ },
            body: Body { kind: KIND_REQ },
        }
    }
    pub fn source_addr(&self) -> Bytes {
        Bytes(handler::source_addr())
    }
    /// the version of the http-request
    pub fn version(&self) -> Bytes {
        Bytes(handler::version())
    }
    pub fn method(&self) -> Bytes {
        Bytes(handler::method())
    }
    pub fn set_method(&self, method: &Bytes) {
        handler::set_method(method);
    }
    pub fn uri(&self) -> Bytes {
        Bytes(handler::uri())
    }
    pub fn set_uri(&self, uri: &Bytes) {
        handler::set_uri(uri);
    }
    pub fn header(&self) -> &Header {
        &self.header
    }
    pub fn body(&self) -> &Body {
        &self.body
    }
}
pub struct Response {
    header: Header,
    body: Body,
}

impl Default for Response {
    fn default() -> Self {
        Self::new()
    }
}

impl Response {
    pub fn new() -> Self {
        Self {
            header: Header { kind: KIND_RES },
            body: Body { kind: KIND_RES },
        }
    }
    pub fn status(&self) -> i32 {
        handler::status_code()
    }
    pub fn set_status(&self, code: i32) {
        handler::set_status_code(code);
    }
    pub fn header(&self) -> &Header {
        &self.header
    }
    pub fn body(&self) -> &Body {
        &self.body
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_empty() {
        let b = Bytes(b"".to_vec().into_boxed_slice());
        assert_eq!(b.is_empty(), true);
    }

    #[test]
    fn test_bytes_from_str() {
        let val = "test";
        let b = Bytes::from(val);
        assert_eq!(val, b.as_str());
        assert_eq!(val, format!("{}", b));
    }
    #[test]
    fn test_bytes_from_u8() {
        let val = b"test";
        let b = Bytes::from(val.as_slice());
        assert_eq!(val, b.as_ref());
    }
    #[test]
    fn test_req() {
        let r = Request::new();
        let sut = r.method();
        assert_eq!("GET", sut.as_str());
    }
}
