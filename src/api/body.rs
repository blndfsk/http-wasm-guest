use crate::api::Bytes;
/// Represents the body of an HTTP message, providing read and write access to payload bytes.
///
/// Implementations are backed by the host environment and may reflect streaming or buffered
/// data depending on enabled features. For large bodies, reading can be memory-intensive.
pub trait Body {
    /// Reads the entire body content as bytes.
    ///
    /// This returns a snapshot of the current body. For streamed bodies, the host may buffer
    /// content before exposing it to the guest, depending on feature flags and host behavior.
    fn read(&self) -> Bytes;
    /// Writes data to the body, replacing any existing content.
    ///
    /// This overwrites the current body with the provided bytes. Use this to fully replace
    /// the payload; append-style behavior is not guaranteed by the host.
    fn write(&self, body: &[u8]);
}
