use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    ops::Deref,
    str::{Utf8Error, from_utf8},
    string::FromUtf8Error,
};

pub mod feature;
mod handler;
pub mod log;

pub fn get_config() -> Result<String, FromUtf8Error> {
    String::from_utf8(handler::get_config())
}

static KIND_REQ: i32 = 0;
static KIND_RES: i32 = 1;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct Bytes(Box<[u8]>);
impl Bytes {
    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        from_utf8(&self.0)
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
        let s = match self.to_str() {
            Ok(res) => res,
            Err(err) => &err.to_string(),
        };
        write!(f, "{}", &s)
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
    pub fn get(&self) -> HashMap<Bytes, Vec<Bytes>> {
        let headers = self.names();
        let mut result = HashMap::with_capacity(headers.len());
        for key in headers {
            let values = self.values(&key);
            result.insert(key, values);
        }
        result
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
        assert_eq!(val, b.to_str().unwrap());
        assert_eq!(val, format!("{}", b));
    }
    #[test]
    fn test_bytes_from_u8() {
        let val = b"test";
        let b = Bytes::from(val.as_slice());
        assert_eq!(val, b.as_ref());
    }

    #[test]
    fn test_bytes_to_str_invalid() {
        let val = b"\xFF\xFF";
        let b = Bytes::from(val.as_slice());
        assert!(b.to_str().is_err());
    }
    #[test]
    fn test_req() {
        let r = Request::new();
        let sut = r.method();
        assert_eq!("GET", sut.to_str().unwrap());
    }

    #[test]
    fn test_header_names() {
        let r = Request::new();
        let sut = r.header().names();
        assert_eq!(2, sut.len());
        assert!(sut.contains(&Bytes::from("X-FOO")));
    }
    #[test]
    fn test_header_values() {
        let r = Request::new();
        let sut = r.header().values(&Bytes::from("value"));
        assert!(!sut.is_empty());
        assert!(sut.contains(&Bytes::from("test1")));
    }
    #[test]
    fn test_header_get() {
        let r = Request::new();
        let sut = r.header().get();
        let h1 = Bytes::from("X-FOO");
        let h2 = Bytes::from("x-bar");
        assert!(!sut.is_empty());
        assert!(sut.contains_key(&h1));
        assert!(sut.contains_key(&h2));
        assert_eq!(sut.len(), 2);
        assert_eq!(sut.get(&h1), Some(&vec!(Bytes::from("test1"))));
    }
    #[test]
    fn test_body() {
        let r = Response::new();
        let sut = r.body().read();
        assert!(!sut.is_empty());
        assert!(sut.starts_with(b"<html>"));
    }
}
