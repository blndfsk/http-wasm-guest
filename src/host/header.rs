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
    /// The `kind` value is used by the host API to distinguish between
    /// request and response headers.
    pub(crate) fn new(kind: i32) -> Self {
        Self(kind)
    }

    /// Returns an iterator over all header names as raw bytes without allocating into a vector.
    ///
    /// Header names are returned in the order provided by the host runtime.
    /// This method is zero-allocation and returns an iterator that yields each
    /// header name as `Bytes`. For heap-allocated results, use [`names`](Header::names).
    pub fn names_iter(&self) -> impl Iterator<Item = Bytes> + use<'_> {
        handler::header_names(self.0).into_iter().map(Bytes::from)
    }

    /// Returns all header names as raw bytes, allocating into a vector.
    ///
    /// Header names are returned in the order provided by the host runtime.
    /// This method collects results into a `Vec`, which allocates heap memory.
    /// Use [`names_iter`](Header::names_iter) for zero-allocation access.
    pub fn names(&self) -> Vec<Bytes> {
        self.names_iter().collect()
    }

    /// Returns an iterator over all values for the given header name without allocating into a vector.
    ///
    /// The `name` is matched by the host according to its header normalization
    /// rules (often case-insensitive). This method is zero-allocation and returns
    /// an iterator that yields each header value as `Bytes`. For heap-allocated results,
    /// use [`values`](Header::values).
    pub fn values_iter(&self, name: &[u8]) -> impl Iterator<Item = Bytes> + use<'_> {
        handler::header_values(self.0, name).into_iter().map(Bytes::from)
    }

    /// Return the first value for the given header name, if present.
    pub fn get(&self, name: &[u8]) -> Option<Bytes> {
        self.values_iter(name).next()
    }

    /// Returns all values for the given header name, allocating into a vector.
    ///
    /// The `name` is matched by the host according to its header normalization
    /// rules (often case-insensitive). This method collects results into a `Vec`,
    /// which allocates heap memory. Use [`values_iter`](Header::values_iter) for
    /// zero-allocation access.
    pub fn values(&self, name: &[u8]) -> Vec<Bytes> {
        self.values_iter(name).collect()
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

    /// Return all headers as an iterator of names to value lists.
    ///
    /// This returns an iterator over all header entries. Each entry contains
    /// the header name paired with a vector containing its associated values.
    /// For zero-allocation access, use [`names_iter`](Header::names_iter) and
    /// [`values_iter`](Header::values_iter).
    pub fn entries_iter(&self) -> impl Iterator<Item = (Bytes, Vec<Bytes>)> + '_ {
        self.names_iter().map(|name| {
            let values: Vec<Bytes> = self.values_iter(&name).collect();
            (name, values)
        })
    }

    /// Return all headers as a map of names to value lists.
    ///
    /// This collects all names and then queries each set of values, allocating
    /// into a `HashMap` and multiple `Vec`s for the values. Each header name is
    /// paired with a vector containing its associated values. Use
    /// [`entries_iter`](Header::entries_iter) for zero-allocation access.
    pub fn entries(&self) -> HashMap<Bytes, Vec<Bytes>> {
        self.entries_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_get_existing() {
        let header = Header::new(0);
        // The mock has "X-FOO" header with value "test1"
        let value = header.get(b"X-FOO");
        assert!(value.is_some());
        assert_eq!(&value.unwrap(), b"test1");
    }

    #[test]
    fn header_get_nonexistent() {
        let header = Header::new(0);
        let value = header.get(b"X-NONEXISTENT");
        assert!(value.is_none());
    }

    #[test]
    fn header_get_all_single_value() {
        let header = Header::new(0);
        let values = header.values(b"X-FOO");
        assert_eq!(values.len(), 1);
        assert_eq!(values[0], b"test1");
    }

    #[test]
    fn header_get_all_multiple_values() {
        let header = Header::new(0);
        // The mock has "x-bar" with values "test2" and "test3"
        let values = header.values(b"x-bar");
        assert_eq!(values.len(), 2);
        assert_eq!(values[0], "test2");
        assert_eq!(values[1], b"test3");
    }

    #[test]
    fn header_names() {
        let header = Header::new(0);
        let names = header.names();
        // The mock provides: X-FOO, x-bar, x-baz
        assert_eq!(names.len(), 3);
    }

    #[test]
    fn header_values_map() {
        let header = Header::new(0);
        let values_map = header.entries();
        // Should have 3 distinct header names
        assert_eq!(values_map.len(), 3);
        // X-FOO should have 1 value
        assert_eq!(values_map.get(&Bytes::from("X-FOO")).unwrap().len(), 1);
        // x-bar should have 2 values
        assert_eq!(values_map.get(&Bytes::from(b"x-bar")).map(|v| v.len()), Some(2));
    }

    #[test]
    fn header_values_iter() {
        let header = Header::new(0);
        let values = header.entries_iter();

        assert_eq!(values.count(), 3);
    }

    #[test]
    fn header_operations_with_bytes() {
        let header = Header::new(0);
        let name = Bytes::from("x-bar");
        let values = header.values(&name);
        assert!(!values.is_empty());
    }

    #[test]
    fn header_values_map_with_duplicate_values() {
        let header = Header::new(0);
        let values_map = header.entries();

        //should have 2 values
        let dup_values = values_map.get(&Bytes::from("x-baz")).unwrap();
        assert_eq!(dup_values.len(), 2);
    }
}
