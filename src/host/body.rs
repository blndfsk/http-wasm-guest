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
