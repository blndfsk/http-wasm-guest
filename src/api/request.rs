use crate::api::{Bytes, body::Body, header::Header};

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
    /// Returns a reference to the request headers.
    fn header(&self) -> &dyn Header;
    /// Returns a reference to the request body.
    fn body(&self) -> &dyn Body;
}
