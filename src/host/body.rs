use crate::host::{Bytes, handler};

/// Handle for accessing and mutating an HTTP body stream.
///
/// A `Body` is tied to a specific request or response context, depending on how
/// it is constructed. Use it to read the full body or write a new one.
///
/// # Cost model
///
/// [`read`](Body::read) drains the body through repeated host calls of at most
/// 2048 bytes each, and returns an owned [`Bytes`] copy — not a view into guest
/// memory. [`write`](Body::write) passes your slice straight to the host in a
/// single call without any guest-side allocation.
pub struct Body(i32);
impl Body {
    /// Create a new body handle for the given kind.
    pub(crate) fn new(kind: i32) -> Self {
        Self(kind)
    }

    /// Read the entire body into memory and return it as owned [`Bytes`].
    ///
    /// The body is drained in chunks of at most 2048 bytes through repeated
    /// `read_body` host calls until the host reports EOF: a large body therefore
    /// costs one host call per 2048-byte chunk plus amortized heap growth to hold
    /// the full payload (capped at just under 16 MB). The returned [`Bytes`] owns
    /// its data; it is not a view into guest memory.
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

    /// Write the provided bytes as the body.
    ///
    /// Per the [HTTP Handler ABI](https://http-wasm.io/http-handler-abi/),
    /// `write_body` is stateful: the first call in `handle_request` or
    /// `handle_response` overwrites any existing body, and subsequent calls
    /// append to it. The host reads your slice directly from guest memory in a
    /// single call; no guest-side allocation or copy is made.
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
        assert!(String::from_utf8_lossy(&content).contains("html"));
    }

    #[test]
    fn body_read_response() {
        let body = Body::new(0);
        let content = body.read();
        assert!(content.is_empty());
    }
}
