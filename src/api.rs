use std::collections::HashMap;

pub use bytes::Bytes;

mod bytes;

/// Trait representing an HTTP request in the guest environment.
pub trait Request {
    /// Returns the source address of the client (IP and port).
    fn source_addr(&self) -> Bytes;
    /// Returns the HTTP protocol version (e.g., "HTTP/1.1").
    fn version(&self) -> Bytes;
    /// Returns the HTTP method (e.g., "GET", "POST").
    fn method(&self) -> Bytes;
    /// Sets the HTTP method for the request.
    fn set_method(&self, method: &[u8]);
    /// Returns the full request URI.
    fn uri(&self) -> Bytes;
    /// Sets the request URI.
    fn set_uri(&self, uri: &[u8]);
    /// Returns a reference to the headers.
    fn header(&self) -> &dyn Header;
    /// Returns a reference to the body.
    fn body(&self) -> &dyn Body;
}

/// Trait representing an HTTP response in the guest environment.
pub trait Response {
    /// Returns the HTTP status code of the response.
    fn status(&self) -> i32;
    /// Sets the HTTP status code of the response.
    fn set_status(&self, code: i32);
    /// Returns a reference to the headers.
    fn header(&self) -> &dyn Header;
    /// Returns a reference to the body.
    fn body(&self) -> &dyn Body;
}

/// Trait for manipulating HTTP headers on requests and responses.
pub trait Header {
    /// Returns all header names present.
    fn names(&self) -> Vec<Bytes>;
    /// Returns all values for a given header name.
    fn values(&self, name: &[u8]) -> Vec<Bytes>;
    /// Sets a header to a single value, replacing any existing values.
    fn set(&self, name: &[u8], value: &[u8]);
    /// Adds a header value, preserving any existing values.
    fn add(&self, name: &[u8], value: &[u8]);
    /// Removes all values for a given header name.
    fn remove(&self, name: &[u8]);
    /// Returns all headers as a map of names to their values.
    fn get(&self) -> HashMap<Bytes, Vec<Bytes>>;
}

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