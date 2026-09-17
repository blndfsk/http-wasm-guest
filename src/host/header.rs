use std::collections::HashMap;

use crate::host::{Bytes, handler};

/// Handle for accessing and mutating HTTP headers.
///
/// A `Header` is scoped to either the request or response, depending on how it
/// is constructed.
///
/// # Cost model
///
/// Every read issues a host call that writes NUL-terminated data into a shared
/// 2048-byte guest buffer (larger payloads are retried once against an
/// exactly-sized heap allocation, capped at just under 16 MB). Because that
/// buffer is reused for every subsequent host call, results are returned as
/// **owned** [`Bytes`] copies rather than zero-copy views: expect one heap
/// allocation per name or value plus the backing `Vec`. The `*_iter` forms skip
/// only the final collection step; they do not avoid the per-item allocations.
///
/// Per the [HTTP Handler ABI](https://http-wasm.io/http-handler-abi/), header
/// names are reported in lowercase and name lookups are case-insensitive.
pub struct Header(i32);
impl Header {
    /// Create a header handle for a specific host-defined kind.
    ///
    /// The `kind` value is used by the host API to distinguish between
    /// request and response headers.
    pub(crate) fn new(kind: i32) -> Self {
        Self(kind)
    }

    /// Returns an iterator over all header names as raw bytes.
    ///
    /// Header names are returned in lowercase, in the order provided by the
    /// host runtime. Each name is yielded as an owned [`Bytes`] copied out of
    /// the shared guest buffer, so each item costs one heap allocation even
    /// though no `Vec` is collected up front. Use [`names`](Header::names) to
    /// collect them into a `Vec`.
    pub fn names_iter(&self) -> impl Iterator<Item = Bytes> + use<'_> {
        handler::header_names(self.0).into_iter().map(Bytes::from)
    }

    /// Returns all header names as raw bytes in a `Vec`.
    ///
    /// Costs one host call, one heap allocation per name, plus the `Vec`
    /// itself. Use [`names_iter`](Header::names_iter) if you only need to
    /// iterate without collecting.
    pub fn names(&self) -> Vec<Bytes> {
        self.names_iter().collect()
    }

    /// Returns an iterator over all values for the given header name.
    ///
    /// The `name` is matched case-insensitively by the host. If the header does
    /// not exist, the iterator yields nothing. Each value is yielded as an owned
    /// [`Bytes`] copied out of the shared guest buffer (one heap allocation per
    /// value); note that all values are read and allocated even if you only
    /// consume some. Use [`values`](Header::values) to collect them into a `Vec`.
    pub fn values_iter(&self, name: &[u8]) -> impl Iterator<Item = Bytes> + use<'_> {
        handler::header_values(self.0, name).into_iter().map(Bytes::from)
    }

    /// Return the first value for the given header name, if present.
    ///
    /// Note this still performs a full lookup: all values of the header are read
    /// from the host and allocated before only the first is returned.
    pub fn get(&self, name: &[u8]) -> Option<Bytes> {
        self.values_iter(name).next()
    }

    /// Returns all values for the given header name in a `Vec`.
    ///
    /// The `name` is matched case-insensitively by the host. Costs one host
    /// call, one heap allocation per value, plus the `Vec` itself. Use
    /// [`values_iter`](Header::values_iter) if you only need to iterate without
    /// collecting.
    pub fn values(&self, name: &[u8]) -> Vec<Bytes> {
        self.values_iter(name).collect()
    }

    /// Set a header value, replacing all existing values of the given name.
    ///
    /// The host traps if it fails to set the header. Matching is
    /// case-insensitive; no heap allocation is made by the guest.
    pub fn set(&self, name: &[u8], value: &[u8]) {
        handler::set_header(self.0, name, value);
    }

    /// Add an additional value for a header name (appending to any existing values).
    ///
    /// The host traps if it fails to add the header. No heap allocation is made
    /// by the guest.
    pub fn add(&self, name: &[u8], value: &[u8]) {
        handler::add_header_value(self.0, name, value);
    }

    /// Remove a header and all of its values.
    ///
    /// The host traps if it fails to remove the header. No heap allocation is
    /// made by the guest.
    pub fn remove(&self, name: &[u8]) {
        handler::remove_header(self.0, name);
    }

    /// Return all headers as an iterator of `(name, values)` pairs.
    ///
    /// This issues one host call to enumerate names and then one further host
    /// call per distinct name to fetch its values, so the cost is O(number of
    /// headers) host calls plus a heap allocation for every name and value. Use
    /// this only when you need both the name and all of its values; otherwise
    /// prefer [`names_iter`](Header::names_iter) and
    /// [`values_iter`](Header::values_iter).
    pub fn entries_iter(&self) -> impl Iterator<Item = (Bytes, Vec<Bytes>)> + '_ {
        self.names_iter().map(|name| {
            let values: Vec<Bytes> = self.values_iter(&name).collect();
            (name, values)
        })
    }

    /// Return all headers as a map of names to value lists.
    ///
    /// Costs the same O(number of headers) host calls as
    /// [`entries_iter`](Header::entries_iter), plus heap allocations for the
    /// `HashMap` and one `Vec` per header. Use
    /// [`entries_iter`](Header::entries_iter) if you only need to iterate
    /// without collecting.
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
        assert_eq!(value.unwrap(), "test1");
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
        assert_eq!(values[0], "test1");
    }

    #[test]
    fn header_get_all_multiple_values() {
        let header = Header::new(0);
        // The mock has "x-bar" with values "test2" and "test3"
        let values = header.values(b"x-bar");
        assert_eq!(values.len(), 2);
        assert_eq!(values[0], "test2");
        assert_eq!(values[1], "test3");
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
        assert_eq!(values_map.get(&Bytes::from_static(b"x-bar")).map(|v| v.len()), Some(2));
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
