//! Test fixture constants for mock FFI behaviors.

#[cfg(test)]
pub(crate) mod kinds {
    /// Mock kind: Returns full buffers indefinitely without EOF
    /// Used to test MAX_BODY_SIZE enforcement
    pub(crate) const OVERSIZED_BODY: i32 = 99;
}
