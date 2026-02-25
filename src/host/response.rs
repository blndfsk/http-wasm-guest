use crate::host::{Body, Header, handler};
/// Handle for accessing and mutating the current HTTP response.
pub struct Response {
    header: Header,
    body: Body,
}
static KIND_RES: i32 = 1;

impl Response {
    /// Creates a new `Response` instance with header and body handles.
    pub(crate) fn new() -> Self {
        Self { header: Header::kind(KIND_RES), body: Body::kind(KIND_RES) }
    }
    /// Return the current response status code.
    pub fn status(&self) -> i32 {
        handler::status_code()
    }

    /// Set the response status code.
    pub fn set_status(&self, code: i32) {
        handler::set_status_code(code);
    }

    /// Return a handle for accessing and mutating response headers.
    pub fn header(&self) -> &Header {
        &self.header
    }

    /// Return a handle for reading or writing the response body.
    pub fn body(&self) -> &Body {
        &self.body
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_body() {
        let r = Response::new();
        let sut = r.body().read();
        assert!(!sut.is_empty());
        assert!(sut.starts_with(b"<html>"));
    }

    #[test]
    fn response_status() {
        let response = Response::new();
        // The mock returns 200
        assert_eq!(response.status(), 200);
    }

    #[test]
    fn response_set_status() {
        let response = Response::new();
        // Should not panic - mock accepts any status
        response.set_status(404);
    }

    #[test]
    fn response_header_access() {
        let response = Response::new();
        let header = response.header();
        // Response headers use kind=1, should still work
        let _ = header.names();
    }

    #[test]
    fn response_body_read() {
        let response = Response::new();
        let body = response.body();
        let content = body.read();
        // The mock returns HTML content
        assert!(!content.is_empty());
    }

    #[test]
    fn response_body_write() {
        let response = Response::new();
        let body = response.body();
        // Should not panic - mock accepts any body
        body.write(b"<html><body>Custom Response</body></html>");
    }
}
