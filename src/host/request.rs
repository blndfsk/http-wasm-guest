use crate::{
    api,
    host::{Body, Bytes, Header, Message, handler},
};

impl api::Request for Message {
    fn source_addr(&self) -> Bytes {
        Bytes::from(handler::source_addr())
    }

    fn version(&self) -> Bytes {
        Bytes::from(handler::version())
    }

    fn method(&self) -> Bytes {
        Bytes::from(handler::method())
    }

    fn set_method(&self, method: &[u8]) {
        handler::set_method(method);
    }

    fn uri(&self) -> Bytes {
        Bytes::from(handler::uri())
    }

    fn set_uri(&self, uri: &[u8]) {
        handler::set_uri(uri);
    }

    fn header(&self) -> &Box<dyn Header + 'static> {
        &self.header
    }

    fn body(&self) -> &Box<dyn Body + 'static> {
        &self.body
    }
}

#[cfg(test)]
mod tests {

    use crate::api::Request;

    use super::*;

    #[test]
    fn test_req() {
        let r = Message::new(0);
        let sut = r.method();
        assert_eq!("GET", sut.to_str().unwrap());
    }

    #[test]
    fn test_header_names() {
        let r = Message::new(0);
        let sut = r.header().names();
        assert_eq!(2, sut.len());
        assert_eq!(sut, vec![Bytes::from("X-FOO"), Bytes::from("x-bar")]);
    }
    #[test]
    fn test_header_values() {
        let r = Message::new(0);
        let sut = r.header().values(&Bytes::from("value"));
        assert!(!sut.is_empty());
        assert!(sut.contains(&Bytes::from("test1")));
    }
    #[test]
    fn test_header_get() {
        let r = Message::new(0);
        let sut = r.header().get();
        let h1 = Bytes::from("X-FOO");
        let h2 = Bytes::from("x-bar");
        assert!(!sut.is_empty());
        assert!(sut.contains_key(&h1));
        assert!(sut.contains_key(&h2));
        assert_eq!(sut.len(), 2);
        assert_eq!(sut.get(&h1), Some(&vec!(Bytes::from("test1"))));
    }
    #[test]
    fn test_version() {
        let r = Message::new(0);
        let sut = r.version();
        assert!(!sut.is_empty());
        assert_eq!(sut.as_ref(), b"HTTP/2.0");
    }
}
