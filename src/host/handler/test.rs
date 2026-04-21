//! Test fixture constants for mock FFI behaviors.

#[cfg(test)]
pub(crate) mod kinds {
    /// Mock kind: Returns buffer indefinitely without EOF
    pub(crate) const BODY_WITHOUT_EOF: i32 = 97;
    /// Mock kind: Returns empty buffers indefinitely without EOF
    pub(crate) const EMPTY_BODY_WITHOUT_EOF: i32 = 98;
    /// Mock kind: Returns full buffers indefinitely without EOF
    /// Used to test MAX_BODY_SIZE enforcement
    pub(crate) const OVERSIZED_BODY: i32 = 99;
}
