use std::{
    fmt::Display,
    ops::Deref,
    str::{Utf8Error, from_utf8},
};

/// A wrapper around a byte array that provides convenience methods for handling binary data.
///
/// `Bytes` is used throughout the http-wasm API to represent string and binary data from
/// HTTP requests and responses. It provides methods to convert to UTF-8 strings and
/// implements common traits for easy manipulation.
///
/// # Examples
///
/// ```rust
/// use http_wasm_guest::host::Bytes;
///
/// // Create from string
/// let bytes = Bytes::from("hello world");
/// assert_eq!(bytes.to_str().unwrap(), "hello world");
///
/// // Create from byte slice
/// let bytes = Bytes::from(b"binary data".as_slice());
/// assert_eq!(bytes.len(), 11);
///
/// // Display as string (handles invalid UTF-8 gracefully)
/// println!("{}", bytes);
/// ```
#[derive(PartialEq, Eq, Clone, Debug, Hash, Default)]
pub struct Bytes(Box<[u8]>);
impl Bytes {
    /// Converts the bytes to a string slice if they contain valid UTF-8.
    ///
    /// # Returns
    ///
    /// Returns a `Result<&str, Utf8Error>` where:
    /// - `Ok(&str)`: A string slice if the bytes are valid UTF-8
    /// - `Err(Utf8Error)`: If the bytes don't form valid UTF-8
    ///
    /// # Example
    ///
    /// ```rust
    /// # use http_wasm_guest::host::Bytes;
    /// let bytes = Bytes::from("hello");
    /// assert_eq!(bytes.to_str().unwrap(), "hello");
    /// ```
    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        from_utf8(&self.0)
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
impl From<&str> for Bytes {
    fn from(value: &str) -> Self {
        Self(value.as_bytes().to_vec().into_boxed_slice())
    }
}
impl From<Vec<u8>> for Bytes {
    fn from(value: Vec<u8>) -> Self {
        Self(value.into_boxed_slice())
    }
}
impl From<&[u8]> for Bytes {
    fn from(value: &[u8]) -> Self {
        Self(value.to_vec().into_boxed_slice())
    }
}
impl From<Box<[u8]>> for Bytes {
    fn from(value: Box<[u8]>) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_empty() {
        let b = Bytes::from("");
        assert!(b.is_empty());
    }

    #[test]
    fn test_bytes_from_str() {
        let val = "test";
        let b = Bytes::from(val);
        assert_eq!(val, b.to_str().unwrap());
        assert_eq!(val, format!("{b}"));
    }
    #[test]
    fn test_bytes_from_u8() {
        let val = b"test";
        let b = Bytes::from(val.as_slice());
        assert_eq!(val, b.as_ref());
    }

    #[test]
    fn test_bytes_to_str_invalid() {
        let val = b"\xFF\xFF";
        let b = Bytes::from(val.as_slice());
        assert!(b.to_str().is_err());
    }
}
