use crate::host::{Body, Header, handler};
/// Handle for accessing and mutating the current HTTP response.
pub struct Response {
    header: Header,
    body: Body,
}
static KIND_RES: i32 = 1;

impl Default for Response {
    fn default() -> Self {
        Self { header: Header::kind(KIND_RES), body: Body::kind(KIND_RES) }
    }
}
impl Response {
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
        let r = Response::default();
        let sut = r.body().read();
        assert!(!sut.is_empty());
        assert!(sut.starts_with(b"<html>"));
    }
}
