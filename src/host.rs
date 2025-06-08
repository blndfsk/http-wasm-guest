use std::string::FromUtf8Error;

pub(crate) mod handler;

pub fn get_config() -> Result<String, FromUtf8Error> {
    String::from_utf8(handler::get_config())
}

/**
 * enables the specified Features on the host.
 *
 * https://http-wasm.io/http-handler-abi/#enable_features
 *
 */
pub fn enable_feature(feature: crate::Feature) -> i32 {
    handler::enable_feature(feature.0)
}

pub mod log {
    pub enum Level {
        Debug = -1,
        Info = 0,
        Warn = 1,
        Error = 2,
        None = 3,
    }
    use super::handler;

    ///writes a message to the host's logs at the given level.
    pub fn write(level: Level, message: &str) {
        if message.is_empty() {
            return;
        }
        handler::log(level as i32, message.as_bytes());
    }
    pub fn enabled(level: Level) -> bool {
        handler::log_enabled(level as i32)
    }
}

static KIND_REQ: i32 = 0;
static KIND_RES: i32 = 1;

pub struct Header {
    kind: i32,
}
impl Header {
    pub fn names(&self) -> Vec<Box<[u8]>> {
        handler::header_names(self.kind)
    }
    pub fn values(&self, name: &[u8]) -> Vec<Box<[u8]>> {
        handler::header_values(self.kind, name)
    }
    pub fn set(&self, name: &[u8], value: &[u8]) {
        handler::set_header(self.kind, name, value);
    }
    pub fn add(&self, name: &[u8], value: &[u8]) {
        handler::add_header_value(self.kind, name, value);
    }
    pub fn remove(&self, name: &[u8]) {
        handler::remove_header(self.kind, name);
    }
}
pub struct Body {
    kind: i32,
}
impl Body {
    pub fn read(&self) -> Option<Box<[u8]>> {
        handler::body(self.kind)
    }
    pub fn write(&self, body: &[u8]) {
        handler::write_body(self.kind, body);
    }
}

pub struct Request {
    header: Header,
    body: Body,
}

impl Request {
    pub fn new() -> Self {
        Self {
            header: Header { kind: KIND_REQ },
            body: Body { kind: KIND_REQ },
        }
    }
    pub fn source_addr(&self) -> Option<Box<[u8]>> {
        handler::source_addr()
    }
    /// the version of the http-request
    pub fn version(&self) -> Option<Box<[u8]>> {
        handler::version()
    }
    pub fn method(&self) -> Option<Box<[u8]>> {
        handler::method()
    }
    pub fn set_method(&self, method: &[u8]) {
        handler::set_method(method);
    }
    pub fn uri(&self) -> Option<Box<[u8]>> {
        handler::uri()
    }
    pub fn set_uri(&self, uri: &[u8]) {
        handler::set_uri(uri);
    }
    pub fn header(&self) -> &Header {
        &self.header
    }
    pub fn body(&self) -> &Body {
        &self.body
    }
}
pub struct Response {
    header: Header,
    body: Body,
}

impl Response {
    pub fn new() -> Self {
        Self {
            header: Header { kind: KIND_RES },
            body: Body { kind: KIND_RES },
        }
    }
    pub fn status(&self) -> i32 {
        handler::status_code()
    }
    pub fn set_status(&self, code: i32) {
        handler::set_status_code(code);
    }
    pub fn header(&self) -> &Header {
        &self.header
    }
    pub fn body(&self) -> &Body {
        &self.body
    }
}
