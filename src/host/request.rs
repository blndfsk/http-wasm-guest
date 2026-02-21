use crate::host::{Body, Bytes, Header, handler};
/// Handle for accessing and mutating the current HTTP request.
pub struct Request {
    header: Header,
    body: Body,
}
static KIND_REQ: i32 = 0;

impl Default for Request {
    fn default() -> Self {
        Self { header: Header::kind(KIND_REQ), body: Body::kind(KIND_REQ) }
    }
}

impl Request {
    /// Return the client source address (ip:port) as raw bytes.
    ///
    /// ## Example ##
    /// - IPv6 b"[fe80::90f4:16ff:fee0:24b3%enp5s0]:41236"
    /// - IPv4 b"1.1.1.1:12345"
    ///
    /// Supported are both IPv4 and IPv6
    /// ## Note ##
    /// A host who fails to get the remote address will trap (aka panic, "unreachable" instruction).
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
    fn request_method() {
        let request = Request::default();
        let method = request.method();
        // The mock returns "GET"
        assert_eq!(method.to_str().unwrap(), "GET");
    }

    #[test]
    fn request_version() {
        let request = Request::default();
        let version = request.version();
        // The mock returns "HTTP/2.0"
        assert!(!version.is_empty());
        assert_eq!(version.to_str().unwrap(), "HTTP/2.0");
    }

    #[test]
    fn request_uri() {
        let request = Request::default();
        let uri = request.uri();
        // The mock returns "https://test"
        assert!(uri.to_str().unwrap().contains("test"));
    }

    #[test]
    fn request_source_addr() {
        let request = Request::default();
        let addr = request.source_addr();
        // The mock returns "192.168.1.1"
        assert_eq!(addr.to_str().unwrap(), "192.168.1.1");
    }

    #[test]
    fn request_header_access() {
        let request = Request::default();
        let header = request.header();
        // The mock provides headers: X-FOO, x-bar, x-baz
        let names = header.names();
        assert_eq!(names.len(), 3);
    }

    #[test]
    fn request_body_access() {
        let request = Request::default();
        let body = request.body();
        let content = body.read();
        // The mock returns "<html><body>test</body>"
        assert!(content.to_str().unwrap().contains("html"));
    }

    #[test]
    fn request_set_method() {
        let request = Request::default();
        // Should not panic - mock accepts any method
        request.set_method(b"POST");
    }

    #[test]
    fn request_set_uri() {
        let request = Request::default();
        // Should not panic - mock accepts any URI
        request.set_uri(b"/new/path?query=value");
    }
}
