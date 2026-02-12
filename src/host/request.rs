use crate::host::{Body, Bytes, Header, handler};
/// Handle for accessing and mutating the current HTTP request.
pub struct Request {
    header: Header,
    body: Body,
}
static KIND_REQ: i32 = 0;

impl Default for Request {
    fn default() -> Self {
        Self {
            header: Header::kind(KIND_REQ),
            body: Body::kind(KIND_REQ),
        }
    }
}

impl Request {
    /// Return the client source address as raw bytes.
    pub fn source_addr(&self) -> Bytes {
        Bytes::from(handler::source_addr())
    }

    /// Return the HTTP protocol version (for example, `HTTP/1.1`).
    pub fn version(&self) -> Bytes {
        Bytes::from(handler::version())
    }

    /// Return the request method (for example, `GET` or `POST`).
    pub fn method(&self) -> Bytes {
        Bytes::from(handler::method())
    }

    /// Replace the request method with the provided bytes.
    pub fn set_method(&self, method: &[u8]) {
        handler::set_method(method);
    }

    /// Return the request URI as raw bytes.
    pub fn uri(&self) -> Bytes {
        Bytes::from(handler::uri())
    }

    /// Replace the request URI with the provided bytes.
    pub fn set_uri(&self, uri: &[u8]) {
        handler::set_uri(uri);
    }

    /// Return a handle for accessing and mutating request headers.
    pub fn header(&self) -> &Header {
        &self.header
    }

    /// Return a handle for reading or writing the request body.
    pub fn body(&self) -> &Body {
        &self.body
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method() {
        let r = Request::default();
        let sut = r.method();
        assert_eq!("GET", sut.to_str().unwrap());
    }

    #[test]
    fn test_header_names() {
        let r = Request::default();
        let sut = r.header().names();
        assert_eq!(2, sut.len());
        assert_eq!(sut, vec![Bytes::from("X-FOO"), Bytes::from("x-bar")]);
    }
    #[test]
    fn test_header_values() {
        let r = Request::default();
        let sut = r.header().values(&Bytes::from("value"));
        assert!(!sut.is_empty());
        assert!(sut.contains(&Bytes::from("test1")));
    }
    #[test]
    fn test_header_get() {
        let r = Request::default();
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
    fn test_version() {
        let r = Request::default();
        let sut = r.version();
        assert!(!sut.is_empty());
        assert_eq!(sut.as_ref(), b"HTTP/2.0");
    }
}
