use crate::api::{body::Body, header::Header};

/// Represents an HTTP response in the http-wasm guest environment.
///
/// The `Response` struct provides access to all aspects of an HTTP response,
/// including status code, headers, and body. You can modify the response
/// during both request and response processing phases.
///
/// # Examples
///
/// ```no_run
/// use http_wasm_guest::{Guest, host::{Request, Response}, register};
///
/// struct MyPlugin;
///
/// impl Guest for MyPlugin {
///     fn handle_request(&self, _request: Request, response: Response) -> (bool, i32) {
///         // Set response status and headers during request phase
///         response.set_status(200);
///         response.header().set(b"content-type", b"application/json");
///         response.body().write(br#"{"status": "ok"}"#);
///
///         (false, 0) // Stop processing, return this response
///     }
///
///     fn handle_response(&self, _request: Request, response: Response) {
///         // Modify response during response phase
///         response.header().add(b"x-processed-by", b"my-plugin");
///
///         // Check and modify status
///         if response.status() >= 400 {
///             response.body().write(b"Custom error page");
///         }
///     }
/// }
/// ```
pub trait Response {
    /// Returns the HTTP status code of the response.
    ///
    /// # Returns
    ///
    /// The HTTP status code as an `i32` (e.g., 200, 404, 500).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use http_wasm_guest::host::Response;
    /// # let response = Response::default();
    /// let status = response.status();
    /// if status >= 400 {
    ///     println!("Error response: {}", status);
    /// }
    /// ```
    fn status(&self) -> i32;
    /// Sets the HTTP status code of the response.
    ///
    /// # Parameters
    ///
    /// - `code`: The HTTP status code to set (e.g., 200, 404, 500)
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use http_wasm_guest::host::Response;
    /// # let response = Response::default();
    /// response.set_status(404); // Not Found
    /// ```
    fn set_status(&self, code: i32);

    /// Returns a reference to the response headers.
    ///
    /// # Returns
    ///
    /// A reference to the `Header` instance for this response.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use http_wasm_guest::host::Response;
    /// # let response = Response::default();
    /// response.header().set(b"content-type", b"application/json");
    /// ```
    fn header(&self) -> &Box<dyn Header>;

    /// Returns a reference to the response body.
    ///
    /// # Returns
    ///
    /// A reference to the `Body` instance for this response.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use http_wasm_guest::host::Response;
    /// # let response = Response::default();
    /// response.body().write(b"Hello, World!");
    /// ```
    fn body(&self) -> &Box<dyn Body>;
}
