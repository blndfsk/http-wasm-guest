use std::collections::HashMap;

use crate::{
    api::{self, Bytes},
    host::handler,
};

pub(crate) struct Header(pub i32);

impl api::Header for Header {
    fn names(&self) -> Vec<Bytes> {
        handler::header_names(self.0)
            .iter()
            .map(|h| Bytes::from(h.clone()))
            .collect()
    }

    fn values(&self, name: &[u8]) -> Vec<Bytes> {
        handler::header_values(self.0, name)
            .iter()
            .map(|h| Bytes::from(h.clone()))
            .collect()
    }

    fn set(&self, name: &[u8], value: &[u8]) {
        handler::set_header(self.0, name, value);
    }

    fn add(&self, name: &[u8], value: &[u8]) {
        handler::add_header_value(self.0, name, value);
    }

    fn remove(&self, name: &[u8]) {
        handler::remove_header(self.0, name);
    }

    fn get(&self) -> HashMap<Bytes, Vec<Bytes>> {
        let headers = self.names();
        let mut result = HashMap::with_capacity(headers.len());
        for key in headers {
            let values = self.values(&key);
            result.insert(key, values);
        }
        result
    }
}
