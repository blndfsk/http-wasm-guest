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

    /// Return the first value for the given header name, if present.
    ///
    /// The `name` is matched by the host according to its header normalization
    /// rules (often case-insensitive).
    pub fn get(&self, name: &[u8]) -> Option<Bytes> {
        handler::header_values(self.0, name).first().map(|h| Bytes::from(h.clone()))
    }

    /// Return all values for the given header name.
    ///
    /// The `name` is matched by the host according to its header normalization
    /// rules (often case-insensitive).
    pub fn get_all(&self, name: &[u8]) -> Vec<Bytes> {
        handler::header_values(self.0, name).iter().map(|h| Bytes::from(h.clone())).collect()
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
    pub fn values(&self) -> HashMap<Bytes, Vec<Bytes>> {
        let headers = self.names();
        let mut result: HashMap<Bytes, Vec<Bytes>> = HashMap::with_capacity(headers.len());
        for key in headers {
            let mut values = self.get_all(&key);
            match result.get_mut(&key) {
                Some(val) => {
                    val.append(&mut values);
                }
                None => {
                    result.insert(key, values);
                }
            }
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all() {
        let h = Header::kind(0);
        let sut = h.get_all(b"x-bar");
        assert!(sut.contains(&Bytes::from("test2")));
    }
    #[test]
    fn test_get_unknown_name() {
        let h = Header::kind(0);
        let sut = h.get(b"x-foo");
        assert_eq!(sut, None);
    }

    #[test]
    fn test_header_names() {
        let h = Header::kind(0);
        let sut = h.names();
        assert_eq!(3, sut.len());
    }
    #[test]
    fn test_header_get_all() {
        let h = Header::kind(0);
        let sut = h.get_all(&Bytes::from("x-bar"));
        assert_eq!(sut.len(), 2);
        assert!(sut.contains(&Bytes::from("test2")));
    }
    #[test]
    fn test_header_values() {
        let h = Header::kind(0);
        let sut = h.values();
        assert_eq!(sut.len(), 3);
        assert_eq!(sut.get(&Bytes::from("X-FOO")), Some(&vec!(Bytes::from("test1"))));
        assert_eq!(sut.get(&Bytes::from("x-bar")).unwrap().len(), 2);
    }
}
