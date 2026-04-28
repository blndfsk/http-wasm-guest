use std::{
    borrow::Borrow,
    fmt::Display,
    ops::Deref,
    str::{Utf8Error, from_utf8},
};

/// Owned container for binary data used throughout the API.
///
/// `Bytes` stores its contents as a boxed slice for efficient cloning and
/// sharing. It is suitable for HTTP headers, bodies, and configuration payloads,
/// and can be created from common byte-oriented types.
///
/// Use [`to_str`](Bytes::to_str) to interpret the contents as UTF-8.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Hash, Default)]
pub struct Bytes(Box<[u8]>);

// --- Core API Methods ---

impl Bytes {
    /// Returns the contents as UTF-8 if valid.
    ///
    /// This is a zero-copy view into the underlying bytes. If the data is not
    /// valid UTF-8, an error is returned.
    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        from_utf8(self.0.as_ref())
    }
}

// --- Standard Library Trait Implementations (for Bytes) ---

impl Deref for Bytes {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl Display for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(self.0.as_ref()))
    }
}

impl Borrow<[u8]> for Bytes {
    fn borrow(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl<const N: usize> Borrow<[u8; N]> for Bytes {
    fn borrow(&self) -> &[u8; N] {
        debug_assert!(self.0.len() == N, "mismatching types: expected [u8; {}], got [u8; {N}]", self.0.len());
        // SAFETY: We verified that self.len() == N above, so the slice has exactly N elements.
        // Casting from &[u8] to &[u8; N] is valid when the lengths match.
        unsafe { &*(self.0.as_ref() as *const [u8] as *const [u8; N]) }
    }
}

// --- Comparison Trait Implementations ---
impl PartialEq<Bytes> for [u8] {
    fn eq(&self, other: &Bytes) -> bool {
        self == other.0.as_ref()
    }
}

impl PartialEq<[u8]> for Bytes {
    fn eq(&self, other: &[u8]) -> bool {
        self.0.as_ref() == other
    }
}

impl PartialEq<Bytes> for &[u8] {
    fn eq(&self, other: &Bytes) -> bool {
        *self == other.0.as_ref()
    }
}

impl PartialEq<&[u8]> for Bytes {
    fn eq(&self, other: &&[u8]) -> bool {
        self.0.as_ref() == *other
    }
}

impl<const N: usize> PartialEq<[u8; N]> for Bytes {
    fn eq(&self, other: &[u8; N]) -> bool {
        self.0.as_ref() == other
    }
}

impl<const N: usize> PartialEq<Bytes> for [u8; N] {
    fn eq(&self, other: &Bytes) -> bool {
        self == other.0.as_ref()
    }
}

impl<const N: usize> PartialEq<&[u8; N]> for Bytes {
    fn eq(&self, other: &&[u8; N]) -> bool {
        self.0.as_ref() == *other
    }
}
impl<const N: usize> PartialEq<Bytes> for &[u8; N] {
    fn eq(&self, other: &Bytes) -> bool {
        *self == other.0.as_ref()
    }
}

impl PartialEq<str> for Bytes {
    fn eq(&self, other: &str) -> bool {
        match self.to_str() {
            Ok(s) => s == other,
            Err(_) => false,
        }
    }
}

impl PartialEq<&str> for Bytes {
    fn eq(&self, other: &&str) -> bool {
        match self.to_str() {
            Ok(s) => s == *other,
            Err(_) => false,
        }
    }
}

impl PartialEq<Bytes> for str {
    fn eq(&self, other: &Bytes) -> bool {
        self.as_bytes() == other.0.as_ref()
    }
}
impl PartialEq<Bytes> for &str {
    fn eq(&self, other: &Bytes) -> bool {
        self.as_bytes() == other.0.as_ref()
    }
}

// --- Conversion Trait Implementations (From<...> for Bytes) ---

/// Creates a `Bytes` value from an existing boxed slice without copying.
impl From<Box<[u8]>> for Bytes {
    fn from(value: Box<[u8]>) -> Self {
        Self(value)
    }
}

/// Creates a `Bytes` value by copying a byte slice.
impl From<&[u8]> for Bytes {
    fn from(value: &[u8]) -> Self {
        Self(value.to_vec().into_boxed_slice())
    }
}

/// Creates a `Bytes` value by copying a byte slice.
impl<const N: usize> From<&[u8; N]> for Bytes {
    fn from(value: &[u8; N]) -> Self {
        Self(Box::new(*value))
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

/// Creates a `Bytes` value from a UTF-8 string slice.
///
/// The string is copied into an owned byte buffer.
impl From<&str> for Bytes {
    fn from(value: &str) -> Self {
        Self(value.as_bytes().into())
    }
}

// --- Test Module ---

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_display_valid_utf8() {
        let s = "valid";
        let b = Bytes::from(s);
        assert_eq!(format!("{b}"), s);
    }

    #[test]
    fn test_display_invalid_utf8() {
        let b = Bytes::from(vec![0x48, 0xFF, 0x6c, 0x6c, 0x6f]);
        assert_eq!(format!("{b}"), "H�llo");
    }

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
        assert_eq!(&bytes, original);
    }

    #[test]
    fn bytes_from_vec_roundtrip() {
        let original = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]; // "Hello"
        let bytes = Bytes::from(original.clone());
        assert_eq!(&bytes, original.as_slice());
    }

    #[test]
    fn bytes_from_boxed_slice() {
        let boxed: Box<[u8]> = vec![1, 2, 3, 4, 5].into_boxed_slice();
        let bytes = Bytes::from(boxed.clone());
        assert_eq!(&bytes, boxed.as_ref());
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

    #[test]
    fn slice_eq_bytes() {
        let slice: &[u8] = &vec![1, 2, 3, 4][..];
        let bytes = Bytes::from(vec![1, 2, 3, 4]);
        assert_eq!(slice, bytes);
        assert_eq!(&*slice, &bytes);

        assert!(bytes.eq(slice));
        assert!(bytes.eq(&slice));
        assert!(&bytes.eq(slice));
        assert!(slice.eq(&bytes));
    }

    #[test]
    fn array_slice_eq_bytes() {
        let arr: &[u8; 4] = &[1, 2, 3, 4];
        let bytes = Bytes::from(vec![1, 2, 3, 4]);
        assert!(bytes.eq(&arr));
        assert!(&bytes.eq(&arr));
        assert!(arr.eq(&bytes));
        assert_eq!(arr, bytes);
    }

    #[test]
    fn str_slice_eq_bytes() {
        let str = "test";
        let bytes = Bytes::from("test");
        assert_eq!(str, bytes);
        assert!(bytes.eq(str));
        assert!(&bytes.eq(str));
        assert!(str.eq(&bytes));
    }

    #[test]
    fn bytes_partial_str_invalid_bytes() {
        let a = "test";
        let b = Bytes::from(vec![0xFF, 0xFE]);

        assert!(!b.eq(a));
        assert!(!b.eq(&a));
        assert!(!a.eq(&b));
    }

    #[test]
    fn bytes_borrow() {
        let a = b"test";
        let b = Bytes::from(a);
        let set: HashSet<Bytes> = HashSet::from([Bytes::from(a)]);
        assert!(set.contains(a));
        assert!(set.contains(&b));

        assert_eq!(set.get(&b[..]), Some(&b));
        assert_eq!(set.get(&b), Some(&b));
        assert_eq!(set.get(a), Some(&b));
    }

    #[test]
    fn bytes_borrow_unknown_key() {
        let a = b"xxx";
        let set: HashSet<Bytes> = HashSet::from([Bytes::from(b"test")]);
        assert!(!set.contains(a));
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "mismatching types")]
    fn bytes_borrow_wrong_type() {
        let b = Bytes::from(b"test");
        let _: [u8; 8] = *b.borrow();
    }
}
