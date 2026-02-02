use crate::api::Bytes;

/// Represents the body of an HTTP request or response.
///
/// The `Body` struct provides methods to read from and write to the HTTP message body.
/// It can handle both text and binary data.
///
/// # Examples
///
/// ```no_run
/// use http_wasm_guest::host::{Request, Response};
///
/// let request = Request::default();
/// let response = Response::default();
///
/// // Read request body
/// let request_body = request.body().read();
/// println!("Request body: {}", request_body);
///
/// // Write response body
/// response.body().write(b"Hello, World!");
///
/// // Write JSON response
/// response.body().write(br#"{"message": "success"}"#);
/// ```
pub trait Body {
    /// Reads the entire body content.
    ///
    /// # Returns
    ///
    /// Returns `Bytes` containing the complete body content.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use http_wasm_guest::host::Request;
    /// let request = Request::default();
    /// let body_content = request.body().read();
    /// println!("Body: {}", body_content);
    /// ```
    fn read(&self) -> Bytes;

    /// Writes data to the body, replacing any existing content.
    ///
    /// # Parameters
    ///
    /// - `body`: The data to write to the body
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use http_wasm_guest::host::Response;
    /// let response = Response::default();
    /// response.body().write(b"Hello, World!");
    /// ```
    fn write(&self, body: &[u8]);
}
