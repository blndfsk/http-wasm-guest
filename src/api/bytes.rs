use std::{
    fmt::Display,
    ops::Deref,
    str::{Utf8Error, from_utf8},
};
/// A container for binary data used throughout the API.
#[derive(PartialEq, Eq, Clone, Debug, Hash, Default)]
pub struct Bytes(Box<[u8]>);

impl Bytes {
    /// Returns the contents as UTF-8 if valid.
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
