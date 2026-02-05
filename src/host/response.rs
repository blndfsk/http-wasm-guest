use crate::{
    api::{Body, Header, Response},
    host::{Message, handler},
};

impl Response for Message {
    fn status(&self) -> i32 {
        handler::status_code()
    }

    fn set_status(&self, code: i32) {
        handler::set_status_code(code);
    }
    fn header(&self) -> &dyn Header {
        self.header.as_ref()
    }

    fn body(&self) -> &dyn Body {
        self.body.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::Response;

    #[test]
    fn test_body() {
        let r = Response::default();
        let sut = r.body().read();
        assert!(!sut.is_empty());
        assert!(sut.starts_with(b"<html>"));
    }
}
