use crate::api::{body::Body, header::Header};

/// Trait representing an HTTP response in the guest environment.
pub trait Response {
    /// Returns the HTTP status code of the response.
    fn status(&self) -> i32;
    /// Sets the HTTP status code of the response.
    fn set_status(&self, code: i32);
    /// Returns a reference to the response headers.
    fn header(&self) -> &dyn Header;
    /// Returns a reference to the response body.
    fn body(&self) -> &dyn Body;
}
