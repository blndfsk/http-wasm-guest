use std::{
    fmt::Display,
    ops::Deref,
    str::{Utf8Error, from_utf8, from_utf8_unchecked},
};
/// Owned container for binary data used throughout the API.
///
/// `Bytes` stores its contents as a boxed slice for efficient cloning and
/// sharing. It is suitable for HTTP headers, bodies, and configuration payloads,
/// and can be created from common byte-oriented types.
///
/// Use [`to_str`](Bytes::to_str) to interpret the contents as UTF-8.
#[derive(PartialEq, Eq, Clone, Debug, Hash, Default)]
pub struct Bytes(Box<[u8]>);

impl Bytes {
    /// Returns the contents as UTF-8 if valid.
    ///
    /// This is a zero-copy view into the underlying bytes. If the data is not
    /// valid UTF-8, an error is returned.
    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        from_utf8(&self.0)
    }
    /// Returns the contents as UTF-8 without validation.
    ///
    /// This is a zero-copy view into the underlying bytes and assumes the data
    /// is valid UTF-8. Use with care if the bytes may be invalid.
    pub fn to_str_unchecked(&self) -> &str {
        unsafe { from_utf8_unchecked(&self.0) }
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
/// Creates a `Bytes` value from a UTF-8 string slice.
///
/// The string is copied into an owned byte buffer.
impl From<&str> for Bytes {
    fn from(value: &str) -> Self {
        Self(value.as_bytes().to_vec().into_boxed_slice())
    }
}
/// Creates a `Bytes` value by taking ownership of a byte vector.
///
/// This avoids an extra copy by converting the `Vec<u8>` into a boxed slice.
impl From<Vec<u8>> for Bytes {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into_boxed_slice())
    }
}
/// Creates a `Bytes` value by copying a byte slice.
impl From<&[u8]> for Bytes {
    fn from(value: &[u8]) -> Self {
        Self(value.to_vec().into_boxed_slice())
    }
}
/// Creates a `Bytes` value from an existing boxed slice without copying.
impl From<Box<[u8]>> for Bytes {
    fn from(value: Box<[u8]>) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn bytes_from_string_slice_roundtrip() {
        let original = "Hello, http-wasm!";
        let bytes = Bytes::from(original);
        assert_eq!(bytes.to_str().unwrap(), original);
    }

    #[test]
    fn bytes_from_byte_slice_roundtrip() {
        let original: &[u8] = b"binary data \x00\x01\x02";
        let bytes = Bytes::from(original);
        assert_eq!(bytes.as_ref(), original);
    }

    #[test]
    fn bytes_from_vec_roundtrip() {
        let original = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]; // "Hello"
        let bytes = Bytes::from(original.clone());
        assert_eq!(bytes.as_ref(), original.as_slice());
    }

    #[test]
    fn bytes_from_boxed_slice() {
        let boxed: Box<[u8]> = vec![1, 2, 3, 4, 5].into_boxed_slice();
        let bytes = Bytes::from(boxed.clone());
        assert_eq!(bytes.as_ref(), boxed.as_ref());
    }

    #[test]
    fn bytes_equality() {
        let a = Bytes::from("test");
        let b = Bytes::from("test");
        let c = Bytes::from("different");

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn bytes_clone() {
        let original = Bytes::from("clone me");
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn bytes_display() {
        let bytes = Bytes::from("display test");
        assert_eq!(format!("{}", bytes), "display test");
    }

    #[test]
    fn bytes_empty() {
        let empty = Bytes::from("");
        assert!(empty.is_empty());
        assert_eq!(empty.len(), 0);
    }

    #[test]
    fn bytes_deref_to_slice() {
        let bytes = Bytes::from("deref");
        let slice: &[u8] = &bytes;
        assert_eq!(slice, b"deref");
    }

    #[test]
    fn bytes_hash_consistency() {
        let a = Bytes::from("hash me");
        let b = Bytes::from("hash me");

        let mut set = HashSet::new();
        set.insert(a.clone());

        assert!(set.contains(&b));
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn bytes_to_str_unchecked_valid() {
        let valid = Bytes::from("valid utf8");
        assert_eq!(valid.to_str_unchecked(), "valid utf8");
    }

    #[test]
    fn bytes_invalid_utf8_to_str() {
        let invalid = Bytes::from(vec![0xFF, 0xFE]);
        assert!(invalid.to_str().is_err());
    }

    #[test]
    fn bytes_display_invalid_utf8() {
        // When displaying invalid UTF-8, it should show the error message
        let invalid = Bytes::from(vec![0xFF, 0xFE]);
        let displayed = format!("{}", invalid);
        // The display should contain error info since it's invalid UTF-8
        assert!(!displayed.is_empty());
    }
}
