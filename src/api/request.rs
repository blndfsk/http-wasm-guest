use crate::api::{Bytes, body::Body, header::Header};

/// Represents an HTTP request in the http-wasm guest environment.
///
/// The `Request` struct provides access to all aspects of an incoming HTTP request,
/// including headers, body, method, URI, and metadata like the client's source address.
/// You can both read from and modify the request during processing.
///
/// # Examples
///
/// ```no_run
/// use http_wasm_guest::{Guest, host::{Request, Response}, register};
///
/// struct MyPlugin;
///
/// impl Guest for MyPlugin {
///     fn handle_request(&self, request: Request, _response: Response) -> (bool, i32) {
///         // Access request properties
///         let method = request.method();
///         let uri = request.uri();
///         let client_addr = request.source_addr();
///
///         // Modify the request
///         request.header().add(b"x-processed-by", b"my-plugin");
///         request.set_method(b"POST");
///
///         // Read request body
///         let body_content = request.body().read();
///
///         (true, 0)
///     }
/// }
/// ```
pub trait Request {
    /// Returns the source address of the client that made the request.
    ///
    /// # Returns
    ///
    /// `Bytes` containing the client's IP address and port (e.g., "192.168.1.1:8080").
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use http_wasm_guest::host::Request;
    /// # let request = Request::default();
    /// let client_addr = request.source_addr();
    /// println!("Request from: {}", client_addr);
    /// ```
    fn source_addr(&self) -> Bytes;

    /// Returns the HTTP protocol version of the request.
    ///
    /// # Returns
    ///
    /// `Bytes` containing the protocol version (e.g., "HTTP/1.1", "HTTP/2.0").
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use http_wasm_guest::host::Request;
    /// # let request = Request::default();
    /// let version = request.version();
    /// println!("HTTP version: {}", version);
    /// ```
    fn version(&self) -> Bytes;

    /// Returns the HTTP method of the request.
    ///
    /// # Returns
    ///
    /// `Bytes` containing the method (e.g., "GET", "POST", "PUT", "DELETE").
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use http_wasm_guest::host::Request;
    /// # let request = Request::default();
    /// let method = request.method();
    /// if method.to_str().unwrap() == "POST" {
    ///     // Handle POST request
    /// }
    /// ```
    fn method(&self) -> Bytes;

    /// Sets the HTTP method of the request.
    ///
    /// # Parameters
    ///
    /// - `method`: The new HTTP method (e.g., b"GET", b"POST")
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use http_wasm_guest::host::Request;
    /// # let request = Request::default();
    /// request.set_method(b"POST");
    /// ```
    fn set_method(&self, method: &[u8]);

    /// Returns the URI of the request.
    ///
    /// # Returns
    ///
    /// `Bytes` containing the full URI including path, query string, and fragment.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use http_wasm_guest::host::Request;
    /// # let request = Request::default();
    /// let uri = request.uri();
    /// println!("Request URI: {}", uri);
    /// ```
    fn uri(&self) -> Bytes;

    /// Sets the URI of the request.
    ///
    /// # Parameters
    ///
    /// - `uri`: The new URI (e.g., b"/api/users?page=1")
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use http_wasm_guest::host::Request;
    /// # let request = Request::default();
    /// request.set_uri(b"/api/v2/users");
    /// ```
    fn set_uri(&self, uri: &[u8]);

    /// Returns a reference to the request headers.
    ///
    /// # Returns
    ///
    /// A reference to the `Header` instance for this request.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use http_wasm_guest::host::Request;
    /// # let request = Request::default();
    /// let content_type = request.header().values(b"content-type");
    /// ```
    fn header(&self) -> &Box<dyn Header>;

    /// Returns a reference to the request body.
    ///
    /// # Returns
    ///
    /// A reference to the `Body` instance for this request.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use http_wasm_guest::host::Request;
    /// # let request = Request::default();
    /// let body_content = request.body().read();
    /// ```
    fn body(&self) -> &Box<dyn Body>;
}
