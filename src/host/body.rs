use crate::host::{Bytes, handler};

/// Handle for accessing and mutating an HTTP body stream.
///
/// A `Body` is tied to a specific request or response context, depending on how
/// it is constructed. Use it to read the full body, stream it in chunks, or
/// write a new one.
///
/// # Cost model
///
/// [`read`](Body::read) drains the body through repeated host calls of at most
/// 2048 bytes each, accumulating the full payload. [`read_iter`](Body::read_iter)
/// performs the same host calls one chunk at a time without accumulating.
/// [`write`](Body::write) issues a single host call.
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
    /// the full payload. Use [`read_iter`](Body::read_iter) to process large
    /// bodies chunk by chunk without accumulating them.
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

    /// Read the body as a stream of owned [`Bytes`] chunks of at most 2048 bytes.
    ///
    /// Each call to [`next`](Iterator::next) issues a single `read_body` host
    /// call and returns the bytes read as an owned [`Bytes`]. Unlike
    /// [`read`](Body::read), the body is not accumulated, so memory usage is
    /// bounded by what the caller retains. The stream ends when the host reports
    /// EOF or returns a zero-length read.
    ///
    /// The same feature-gating requirements as [`read`](Body::read) apply:
    /// `feature::BufferRequest` is required for request bodies and
    /// `feature::BufferResponse` for response bodies.
    pub fn read_iter(&self) -> BodyIter {
        BodyIter { kind: self.0, done: false }
    }

    /// Write the provided bytes as the body.
    ///
    /// Writing is stateful: the first call in `handle_request` or
    /// `handle_response` overwrites any existing body, and subsequent calls
    /// append to it.
    pub fn write(&self, body: &[u8]) {
        handler::write_body(self.0, body);
    }
}

/// Iterator over body chunks produced by [`Body::read_iter`].
///
/// Each item is an owned [`Bytes`] slice of at most 2048 bytes. The iterator
/// yields `None` once the host reports EOF or returns a zero-length read;
/// subsequent calls keep yielding `None`.
pub struct BodyIter {
    kind: i32,
    done: bool,
}

impl Iterator for BodyIter {
    type Item = Bytes;

    fn next(&mut self) -> Option<Bytes> {
        if self.done {
            return None;
        }
        let (done, chunk) = handler::body_chunk(self.kind);
        self.done = done;
        (!chunk.is_empty()).then_some(Bytes::from(chunk))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::handler::test;

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

    #[test]
    fn body_read_iter_terminates_on_eof() {
        let body = Body::new(1);
        let mut iter = body.read_iter();
        assert!(iter.next().is_some());
        // EOF latches: subsequent calls keep returning None
        assert!(iter.next().is_none());
        assert!(iter.next().is_none());
    }

    #[test]
    fn body_read_iter_joins_to_read() {
        let body = Body::new(1);
        let mut joined = Vec::new();
        for chunk in body.read_iter() {
            joined.extend_from_slice(&chunk);
        }
        assert_eq!(Bytes::from(joined), body.read());
    }

    #[test]
    fn body_read_iter_empty() {
        let body = Body::new(0);
        assert_eq!(body.read_iter().count(), 0);
    }

    #[test]
    fn body_read_iter_empty_without_eof() {
        // A zero-length read without EOF terminates the stream
        let body = Body::new(test::kinds::EMPTY_BODY_WITHOUT_EOF);
        assert_eq!(body.read_iter().count(), 0);
    }

    #[test]
    fn body_read_iter_oversized_streams() {
        // OVERSIZED_BODY fills the buffer on every call without EOF
        let body = Body::new(test::kinds::OVERSIZED_BODY);
        let chunks: Vec<Bytes> = body.read_iter().take(3).collect();
        let expected = Bytes::from(vec![b'A'; 2048]);
        assert!(chunks.iter().all(|chunk| chunk == &expected));
    }
}
