use crate::{
    api::{self, Bytes},
    host::handler,
};

pub(crate) struct Body(pub i32);

impl api::Body for Body {
    fn read(&self) -> Bytes {
        Bytes::from(handler::body(self.0))
    }

    fn write(&self, body: &[u8]) {
        handler::write_body(self.0, body);
    }
}
