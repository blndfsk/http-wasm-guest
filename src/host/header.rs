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
    fn header_get_existing() {
        let header = Header::kind(0);
        // The mock has "X-FOO" header with value "test1"
        let value = header.get(b"X-FOO");
        assert!(value.is_some());
        assert_eq!(value.unwrap().to_str().unwrap(), "test1");
    }

    #[test]
    fn header_get_nonexistent() {
        let header = Header::kind(0);
        let value = header.get(b"X-NONEXISTENT");
        assert!(value.is_none());
    }

    #[test]
    fn header_get_all_single_value() {
        let header = Header::kind(0);
        let values = header.get_all(b"X-FOO");
        assert_eq!(values.len(), 1);
        assert_eq!(values[0].to_str().unwrap(), "test1");
    }

    #[test]
    fn header_get_all_multiple_values() {
        let header = Header::kind(0);
        // The mock has "x-bar" with values "test2" and "test3"
        let values = header.get_all(b"x-bar");
        assert_eq!(values.len(), 2);
        assert!(values.iter().any(|v| v.to_str().unwrap() == "test2"));
        assert!(values.iter().any(|v| v.to_str().unwrap() == "test3"));
    }

    #[test]
    fn header_names() {
        let header = Header::kind(0);
        let names = header.names();
        // The mock provides: X-FOO, x-bar, x-baz
        assert_eq!(names.len(), 3);
    }

    #[test]
    fn header_values_map() {
        let header = Header::kind(0);
        let values_map = header.values();
        // Should have 3 distinct header names
        assert_eq!(values_map.len(), 3);
        // X-FOO should have 1 value
        assert_eq!(values_map.get(&Bytes::from("X-FOO")).map(|v| v.len()), Some(1));
        // x-bar should have 2 values
        assert_eq!(values_map.get(&Bytes::from("x-bar")).map(|v| v.len()), Some(2));
    }

    #[test]
    fn header_set() {
        let header = Header::kind(0);
        // Should not panic - mock accepts header modifications
        header.set(b"X-New-Header", b"new-value");
    }

    #[test]
    fn header_add() {
        let header = Header::kind(0);
        // Should not panic - mock accepts header additions
        header.add(b"X-Additional", b"additional-value");
    }

    #[test]
    fn header_remove() {
        let header = Header::kind(0);
        // Should not panic - mock accepts header removals
        header.remove(b"X-FOO");
    }

    #[test]
    fn header_operations_with_bytes() {
        let header = Header::kind(0);
        let name = Bytes::from("x-bar");
        let values = header.get_all(&name);
        assert!(!values.is_empty());
    }
}
