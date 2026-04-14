//! Test fixture constants for mock FFI behaviors.

#[cfg(test)]
pub(crate) mod kinds {
    /// Mock kind: Returns duplicate header names (for deduplication testing)
    /// Returns: X-DUP, X-OTHER, X-DUP
    pub(crate) const DUPLICATE_HEADERS: i32 = 98;

    /// Mock kind: Returns full buffers indefinitely without EOF
    /// Used to test MAX_BODY_SIZE enforcement
    pub(crate) const OVERSIZED_BODY: i32 = 99;
}
