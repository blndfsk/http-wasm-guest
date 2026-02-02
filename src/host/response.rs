use crate::{
    api,
    host::{Body, Header, Message, handler},
};

impl api::Response for Message {
    fn status(&self) -> i32 {
        handler::status_code()
    }

    fn set_status(&self, code: i32) {
        handler::set_status_code(code);
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
    use super::*;
    use crate::api::Response as _;
    #[test]
    fn test_body() {
        let r = Message::new(1);
        let sut = r.body().read();
        assert!(!sut.is_empty());
        assert!(sut.starts_with(b"<html>"));
    }
}
