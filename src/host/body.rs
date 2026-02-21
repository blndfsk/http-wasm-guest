use crate::host::{Bytes, handler};

/// Handle for accessing and mutating an HTTP body stream.
///
/// A `Body` is tied to a specific request or response context, depending on how
/// it is constructed. Use it to read the full buffered body or write a new one.
pub struct Body(i32);

impl Body {
    /// Create a body handle for a specific host-defined kind.
    ///
    /// The `kind` value is provided by the host API to distinguish between
    /// request and response bodies.
    pub fn kind(kind: i32) -> Self {
        Self(kind)
    }

    /// Read the entire body into memory and return it as [`Bytes`].
    ///
    /// This returns the buffered payload when body buffering is enabled by the host.
    pub fn read(&self) -> Bytes {
        Bytes::from(handler::body(self.0))
    }

    /// Replace the body with the provided bytes.
    ///
    /// Use this to set a new payload after inspecting or transforming the original.
    pub fn write(&self, body: &[u8]) {
        handler::write_body(self.0, body);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn body_kind_request() {
        let body = Body::kind(0);
        assert_eq!(body.0, 0);
    }

    #[test]
    fn body_kind_response() {
        let body = Body::kind(1);
        assert_eq!(body.0, 1);
    }

    #[test]
    fn body_read_request() {
        let body = Body::kind(0);
        let content = body.read();
        // Mock returns HTML content
        assert!(!content.is_empty());
        assert!(content.to_str().unwrap().contains("html"));
    }

    #[test]
    fn body_read_response() {
        let body = Body::kind(1);
        let content = body.read();
        assert!(!content.is_empty());
    }

    #[test]
    fn body_write_request() {
        let body = Body::kind(0);
        // Should not panic - mock accepts any body
        body.write(b"new request body");
    }

    #[test]
    fn body_write_response() {
        let body = Body::kind(1);
        // Should not panic - mock accepts any body
        body.write(b"<html><body>Custom Response</body></html>");
    }

    #[test]
    fn body_write_empty() {
        let body = Body::kind(0);
        // Should not panic - mock accepts empty body
        body.write(b"");
    }
}
