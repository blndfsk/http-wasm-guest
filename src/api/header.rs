use std::collections::HashMap;

use crate::api::Bytes;

/// Represents HTTP headers for either a request or response.
///
/// The `Header` struct provides methods to read, write, and manipulate HTTP headers.
/// It supports multiple values per header name (as required by the HTTP specification)
/// and handles header names in a case-insensitive manner.
///
/// # Examples
///
/// ```no_run
/// use http_wasm_guest::host::Request;
///
/// let request = Request::default();
/// let headers = request.header();
///
/// // Add a header
/// headers.add(b"x-custom-header", b"custom-value");
///
/// // Set a header (replaces existing values)
/// headers.set(b"content-type", b"application/json");
///
/// // Get header values
/// let content_types = headers.values(b"content-type");
///
/// // Remove a header
/// headers.remove(b"authorization");
/// ```
pub trait Header {
    /// Returns all header names present in the request or response.
    ///
    /// # Returns
    ///
    /// A vector of `Bytes` containing all header names.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use http_wasm_guest::host::Request;
    /// let request = Request::default();
    /// let header_names = request.header().names();
    /// for name in header_names {
    ///     println!("Header: {}", name);
    /// }
    /// ```
    fn names(&self) -> Vec<Bytes>;

    /// Returns all values for a specific header name.
    ///
    /// # Parameters
    ///
    /// - `name`: The header name to look up (case-insensitive)
    ///
    /// # Returns
    ///
    /// A vector of `Bytes` containing all values for the given header name.
    /// Returns an empty vector if the header doesn't exist.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use http_wasm_guest::host::Request;
    /// let request = Request::default();
    /// let values = request.header().values(b"content-type");
    /// for value in values {
    ///     println!("Content-Type: {}", value);
    /// }
    /// ```
    fn values(&self, name: &[u8]) -> Vec<Bytes>;

    /// Sets a header to a single value, replacing any existing values.
    ///
    /// # Parameters
    ///
    /// - `name`: The header name
    /// - `value`: The header value
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use http_wasm_guest::host::Response;
    /// let response = Response::default();
    /// response.header().set(b"content-type", b"application/json");
    /// ```
    fn set(&self, name: &[u8], value: &[u8]);

    /// Adds a header value, preserving any existing values.
    ///
    /// This is useful for headers that can have multiple values like `Set-Cookie`.
    ///
    /// # Parameters
    ///
    /// - `name`: The header name
    /// - `value`: The header value to add
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use http_wasm_guest::host::Response;
    /// let response = Response::default();
    /// response.header().add(b"set-cookie", b"session=abc123");
    /// response.header().add(b"set-cookie", b"theme=dark");
    /// ```    
    fn add(&self, name: &[u8], value: &[u8]);

    /// Removes all values for a specific header name.
    ///
    /// # Parameters
    ///
    /// - `name`: The header name to remove
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use http_wasm_guest::host::Request;
    /// let request = Request::default();
    /// request.header().remove(b"authorization");
    /// ```    
    fn remove(&self, name: &[u8]);

    /// Returns all headers as a HashMap.
    ///
    /// # Returns
    ///
    /// A `HashMap<Bytes, Vec<Bytes>>` where keys are header names and values
    /// are vectors of header values (since headers can have multiple values).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use http_wasm_guest::host::Request;
    /// let request = Request::default();
    /// let all_headers = request.header().get();
    /// for (name, values) in all_headers {
    ///     println!("Header {}: {:?}", name, values);
    /// }
    /// ```
    fn get(&self) -> HashMap<Bytes, Vec<Bytes>>;
}
