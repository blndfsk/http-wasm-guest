//! Host interface module for http-wasm guest plugins.
//!
//! This module provides the main interface between WebAssembly guest modules and
//! the http-wasm host environment. It contains types and functions for handling
//! HTTP requests and responses, managing features, and logging.
//!
//! # Core Types
//!
//! - [`Request`] - Represents an incoming HTTP request
//! - [`Response`] - Represents an HTTP response
//! - [`Header`] - Provides access to HTTP headers
//! - [`Body`] - Provides access to HTTP message bodies
//! - [`Bytes`] - A wrapper for binary data with UTF-8 conversion utilities
//!
//! # Submodules
//!
//! - [`feature`] - Enable optional host features like body buffering
//! - [`log`] - Logging functionality that routes through the host
//!
//! # Example
//!
//! ```no_run
//! use http_wasm_guest::host::{Request, Response, get_config};
//!
//! fn process_request(request: Request, response: Response) {
//!     // Get plugin configuration
//!     let config = get_config().unwrap_or_default();
//!
//!     // Read request details
//!     let method = request.method();
//!     let uri = request.uri();
//!
//!     // Modify response
//!     response.set_status(200);
//!     response.header().set(b"content-type", b"text/plain");
//!     response.body().write(b"Hello from http-wasm!");
//! }
//! ```

use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    ops::Deref,
    str::{Utf8Error, from_utf8},
    string::FromUtf8Error,
};

pub mod feature;
mod handler;
pub mod log;

/// Retrieves the configuration data provided by the host.
///
/// This function returns the configuration that was passed to the WebAssembly module
/// by the host environment. The configuration is typically provided as a string
/// (often JSON or YAML format) that contains settings for the plugin.
///
/// # Returns
///
/// Returns a `Result<String, FromUtf8Error>` where:
/// - `Ok(String)`: The configuration data as a UTF-8 string
/// - `Err(FromUtf8Error)`: If the configuration data is not valid UTF-8
///
/// # Example
///
/// ```no_run
/// use http_wasm_guest::host::get_config;
///
/// match get_config() {
///     Ok(config) => {
///         // Parse configuration (e.g., JSON)
///         println!("Plugin config: {}", config);
///     }
///     Err(e) => {
///         // Handle invalid UTF-8 configuration
///         eprintln!("Invalid config encoding: {}", e);
///     }
/// }
/// ```
pub fn get_config() -> Result<String, FromUtf8Error> {
    String::from_utf8(handler::get_config())
}

static KIND_REQ: i32 = 0;
static KIND_RES: i32 = 1;

/// A wrapper around a byte array that provides convenience methods for handling binary data.
///
/// `Bytes` is used throughout the http-wasm API to represent string and binary data from
/// HTTP requests and responses. It provides methods to convert to UTF-8 strings and
/// implements common traits for easy manipulation.
///
/// # Examples
///
/// ```rust
/// use http_wasm_guest::host::Bytes;
///
/// // Create from string
/// let bytes = Bytes::from("hello world");
/// assert_eq!(bytes.to_str().unwrap(), "hello world");
///
/// // Create from byte slice
/// let bytes = Bytes::from(b"binary data".as_slice());
/// assert_eq!(bytes.len(), 11);
///
/// // Display as string (handles invalid UTF-8 gracefully)
/// println!("{}", bytes);
/// ```
#[derive(PartialEq, Eq, Clone, Debug, Hash, Default)]
pub struct Bytes(Box<[u8]>);
impl Bytes {
    /// Converts the bytes to a string slice if they contain valid UTF-8.
    ///
    /// # Returns
    ///
    /// Returns a `Result<&str, Utf8Error>` where:
    /// - `Ok(&str)`: A string slice if the bytes are valid UTF-8
    /// - `Err(Utf8Error)`: If the bytes don't form valid UTF-8
    ///
    /// # Example
    ///
    /// ```rust
    /// # use http_wasm_guest::host::Bytes;
    /// let bytes = Bytes::from("hello");
    /// assert_eq!(bytes.to_str().unwrap(), "hello");
    /// ```
    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        from_utf8(&self.0)
    }
}
impl Deref for Bytes {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}
impl Display for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self.to_str() {
            Ok(res) => res,
            Err(err) => &err.to_string(),
        };
        write!(f, "{}", &s)
    }
}
impl From<&str> for Bytes {
    fn from(value: &str) -> Self {
        Self(value.as_bytes().to_vec().into_boxed_slice())
    }
}
impl From<&[u8]> for Bytes {
    fn from(value: &[u8]) -> Self {
        Self(value.to_vec().into_boxed_slice())
    }
}

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
/// let request = Request::new();
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
pub struct Header {
    kind: i32,
}
impl Header {
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
    /// let request = Request::new();
    /// let header_names = request.header().names();
    /// for name in header_names {
    ///     println!("Header: {}", name);
    /// }
    /// ```
    pub fn names(&self) -> Vec<Bytes> {
        handler::header_names(self.kind)
            .iter()
            .map(|h| Bytes(h.clone()))
            .collect()
    }

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
    /// let request = Request::new();
    /// let values = request.header().values(b"content-type");
    /// for value in values {
    ///     println!("Content-Type: {}", value);
    /// }
    /// ```
    pub fn values(&self, name: &[u8]) -> Vec<Bytes> {
        handler::header_values(self.kind, name)
            .iter()
            .map(|h| Bytes(h.clone()))
            .collect()
    }

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
    /// let response = Response::new();
    /// response.header().set(b"content-type", b"application/json");
    /// ```
    pub fn set(&self, name: &[u8], value: &[u8]) {
        handler::set_header(self.kind, name, value);
    }

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
    /// let response = Response::new();
    /// response.header().add(b"set-cookie", b"session=abc123");
    /// response.header().add(b"set-cookie", b"theme=dark");
    /// ```
    pub fn add(&self, name: &[u8], value: &[u8]) {
        handler::add_header_value(self.kind, name, value);
    }

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
    /// let request = Request::new();
    /// request.header().remove(b"authorization");
    /// ```
    pub fn remove(&self, name: &[u8]) {
        handler::remove_header(self.kind, name);
    }

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
    /// let request = Request::new();
    /// let all_headers = request.header().get();
    /// for (name, values) in all_headers {
    ///     println!("Header {}: {:?}", name, values);
    /// }
    /// ```
    pub fn get(&self) -> HashMap<Bytes, Vec<Bytes>> {
        let headers = self.names();
        let mut result = HashMap::with_capacity(headers.len());
        for key in headers {
            let values = self.values(&key);
            result.insert(key, values);
        }
        result
    }
}
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
/// let request = Request::new();
/// let response = Response::new();
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
pub struct Body {
    kind: i32,
}
impl Body {
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
    /// let request = Request::new();
    /// let body_content = request.body().read();
    /// println!("Body: {}", body_content);
    /// ```
    pub fn read(&self) -> Bytes {
        Bytes(handler::body(self.kind))
    }

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
    /// let response = Response::new();
    /// response.body().write(b"Hello, World!");
    /// ```
    pub fn write(&self, body: &[u8]) {
        handler::write_body(self.kind, body);
    }
}

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
pub struct Request {
    header: Header,
    body: Body,
}

impl Default for Request {
    fn default() -> Self {
        Self {
            header: Header { kind: KIND_REQ },
            body: Body { kind: KIND_REQ },
        }
    }
}

impl Request {
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
    /// # let request = Request::new();
    /// let client_addr = request.source_addr();
    /// println!("Request from: {}", client_addr);
    /// ```
    pub fn source_addr(&self) -> Bytes {
        Bytes(handler::source_addr())
    }

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
    /// # let request = Request::new();
    /// let version = request.version();
    /// println!("HTTP version: {}", version);
    /// ```
    pub fn version(&self) -> Bytes {
        Bytes(handler::version())
    }

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
    /// # let request = Request::new();
    /// let method = request.method();
    /// if method.to_str().unwrap() == "POST" {
    ///     // Handle POST request
    /// }
    /// ```
    pub fn method(&self) -> Bytes {
        Bytes(handler::method())
    }

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
    /// # let request = Request::new();
    /// request.set_method(b"POST");
    /// ```
    pub fn set_method(&self, method: &[u8]) {
        handler::set_method(method);
    }

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
    /// # let request = Request::new();
    /// let uri = request.uri();
    /// println!("Request URI: {}", uri);
    /// ```
    pub fn uri(&self) -> Bytes {
        Bytes(handler::uri())
    }

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
    /// # let request = Request::new();
    /// request.set_uri(b"/api/v2/users");
    /// ```
    pub fn set_uri(&self, uri: &[u8]) {
        handler::set_uri(uri);
    }

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
    /// # let request = Request::new();
    /// let content_type = request.header().values(b"content-type");
    /// ```
    pub fn header(&self) -> &Header {
        &self.header
    }

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
    /// # let request = Request::new();
    /// let body_content = request.body().read();
    /// ```
    pub fn body(&self) -> &Body {
        &self.body
    }
}
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
pub struct Response {
    header: Header,
    body: Body,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            header: Header { kind: KIND_RES },
            body: Body { kind: KIND_RES },
        }
    }
}

impl Response {
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
    /// # let response = Response::new();
    /// let status = response.status();
    /// if status >= 400 {
    ///     println!("Error response: {}", status);
    /// }
    /// ```
    pub fn status(&self) -> i32 {
        handler::status_code()
    }

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
    /// # let response = Response::new();
    /// response.set_status(404); // Not Found
    /// ```
    pub fn set_status(&self, code: i32) {
        handler::set_status_code(code);
    }

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
    /// # let response = Response::new();
    /// response.header().set(b"content-type", b"application/json");
    /// ```
    pub fn header(&self) -> &Header {
        &self.header
    }

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
    /// # let response = Response::new();
    /// response.body().write(b"Hello, World!");
    /// ```
    pub fn body(&self) -> &Body {
        &self.body
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_empty() {
        let b = Bytes::from("");
        assert!(b.is_empty());
    }

    #[test]
    fn test_bytes_from_str() {
        let val = "test";
        let b = Bytes::from(val);
        assert_eq!(val, b.to_str().unwrap());
        assert_eq!(val, format!("{b}"));
    }
    #[test]
    fn test_bytes_from_u8() {
        let val = b"test";
        let b = Bytes::from(val.as_slice());
        assert_eq!(val, b.as_ref());
    }

    #[test]
    fn test_bytes_to_str_invalid() {
        let val = b"\xFF\xFF";
        let b = Bytes::from(val.as_slice());
        assert!(b.to_str().is_err());
    }
    #[test]
    fn test_req() {
        let r = Request::default();
        let sut = r.method();
        assert_eq!("GET", sut.to_str().unwrap());
    }

    #[test]
    fn test_header_names() {
        let r = Request::default();
        let sut = r.header().names();
        assert_eq!(2, sut.len());
        assert!(sut.contains(&Bytes::from("X-FOO")));
    }
    #[test]
    fn test_header_values() {
        let r = Request::default();
        let sut = r.header().values(&Bytes::from("value"));
        assert!(!sut.is_empty());
        assert!(sut.contains(&Bytes::from("test1")));
    }
    #[test]
    fn test_header_get() {
        let r = Request::default();
        let sut = r.header().get();
        let h1 = Bytes::from("X-FOO");
        let h2 = Bytes::from("x-bar");
        assert!(!sut.is_empty());
        assert!(sut.contains_key(&h1));
        assert!(sut.contains_key(&h2));
        assert_eq!(sut.len(), 2);
        assert_eq!(sut.get(&h1), Some(&vec!(Bytes::from("test1"))));
    }
    #[test]
    fn test_body() {
        let r = Response::default();
        let sut = r.body().read();
        assert!(!sut.is_empty());
        assert!(sut.starts_with(b"<html>"));
    }

    #[test]
    fn test_version() {
        let r = Request::default();
        let sut = r.version();
        assert!(!sut.is_empty());
        assert_eq!(sut.as_ref(), b"HTTP/2.0");
    }
}
