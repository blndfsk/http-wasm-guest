use std::collections::HashMap;

use crate::api::Bytes;
/// Trait for manipulating HTTP headers on requests and responses.
pub trait Header {
    /// Returns all header names present.
    fn names(&self) -> Vec<Bytes>;
    /// Returns all values for a given header name.
    fn values(&self, name: &[u8]) -> Vec<Bytes>;
    /// Sets a header to a single value, replacing any existing values.
    fn set(&self, name: &[u8], value: &[u8]);
    /// Adds a header value, preserving any existing values.
    fn add(&self, name: &[u8], value: &[u8]);
    /// Removes all values for a given header name.
    fn remove(&self, name: &[u8]);
    /// Returns all headers as a map of names to their values.
    fn get(&self) -> HashMap<Bytes, Vec<Bytes>>;
}
