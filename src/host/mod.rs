/// Host feature flags and enablement helpers.
pub mod feature;
/// Logging helpers that route through the host.
pub mod log;

pub(crate) mod request;
pub(crate) mod response;

mod body;
mod handler;
mod header;

use crate::api::{Body, Bytes, Header};

pub(crate) struct Message {
    header: Box<dyn Header>,
    body: Box<dyn Body>,
}
impl Message {
    pub fn new(kind: i32) -> Self {
        Self {
            header: Box::new(header::Header(kind)),
            body: Box::new(body::Body(kind)),
        }
    }
}
/// Returns the raw configuration bytes provided by the host.
pub fn config() -> Bytes {
    Bytes::from(handler::get_config())
}

#[cfg(test)]
mod tests {}
