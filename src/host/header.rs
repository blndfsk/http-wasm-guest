use std::collections::HashMap;

use crate::host::{Bytes, handler};

/// Handle for accessing and mutating HTTP headers.
///
/// A `Header` is scoped to either the request or response, depending on how it
/// is constructed.
pub struct Header(i32);

impl Header {
    /// Create a header handle for a specific host-defined kind.
    ///
    /// The `kind` value is provided by the host API to distinguish between
    /// request and response headers.
    pub fn kind(kind: i32) -> Self {
        Self(kind)
    }

    /// Return all header names as raw bytes.
    ///
    /// Header names are returned in the order provided by the host runtime.
    pub fn names(&self) -> Vec<Bytes> {
        handler::header_names(self.0).iter().map(|h| Bytes::from(h.clone())).collect()
    }

    /// Return all values for the given header name.
    ///
    /// The `name` is matched by the host according to its header normalization
    /// rules (often case-insensitive).
    pub fn values(&self, name: &[u8]) -> Vec<Bytes> {
        handler::header_values(self.0, name)
            .iter()
            .map(|h| Bytes::from(h.clone()))
            .collect()
    }

    /// Set a header value, replacing any existing values.
    pub fn set(&self, name: &[u8], value: &[u8]) {
        handler::set_header(self.0, name, value);
    }

    /// Add an additional value for a header name.
    pub fn add(&self, name: &[u8], value: &[u8]) {
        handler::add_header_value(self.0, name, value);
    }

    /// Remove a header and all of its values.
    pub fn remove(&self, name: &[u8]) {
        handler::remove_header(self.0, name);
    }

    /// Return all headers as a map of names to value lists.
    ///
    /// This collects all names and then queries each set of values.
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
