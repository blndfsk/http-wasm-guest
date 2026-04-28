use crate::host::{Bytes, handler};

/// Handle for accessing and mutating an HTTP body stream.
///
/// A `Body` is tied to a specific request or response context, depending on how
/// it is constructed. Use it to read the full buffered body or write a new one.
pub struct Body(i32);

impl Body {
    /// Create a new body handle for the given kind.
    pub(crate) fn new(kind: i32) -> Self {
        Self(kind)
    }

    /// Read the entire body into memory and return it as [`Bytes`].
    ///
    /// This returns the buffered payload when body buffering is enabled by the host.
    ///
    /// `feature::BufferRequest` is required to read without consuming the request body.
    /// To enable it, call `admin::enable(BufferRequest)` before returning from handle_request.
    /// Otherwise, the next handler may panic attempting to read the request body because it was already read.
    ///
    /// `feature::BufferResponse` is required to read the response body produced by the next handler defined
    /// on the host inside handle_response. To enable it, call `admin::enable(BufferResponse)` beforehand.
    /// Otherwise, the guest may read EOF because the downstream handler already consumed it.
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
    fn body_read_request() {
        let body = Body::new(1);
        let content = body.read();
        // Mock returns HTML content
        assert!(!content.is_empty());
        assert!(content.to_str().unwrap().contains("html"));
    }

    #[test]
    fn body_read_response() {
        let body = Body::new(0);
        let content = body.read();
        assert!(content.is_empty());
    }
}
