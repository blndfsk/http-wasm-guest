use crate::api::Bytes;
/// Represents the body of an HTTP message for reading and writing data.
pub trait Body {
    /// Reads the entire body content as bytes.
    fn read(&self) -> Bytes;
    /// Writes data to the body, replacing any existing content.
    fn write(&self, body: &[u8]);
}
