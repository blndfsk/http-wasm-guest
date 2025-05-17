use memory::buffer;

pub(crate) mod handler;
pub(crate) mod memory;

pub fn get_config() -> Option<Vec<u8>> {
    handler::get_config(buffer())
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

#[derive(Clone, Copy)]
struct Type(i32);
static TYPE_REQUEST: Type = Type(0);
static TYPE_RESPONSE: Type = Type(1);

pub struct Header(Type);
impl Header {
    pub fn names(&self) -> Vec<Vec<u8>> {
        handler::header_names(buffer(), self.0.0)
    }
    pub fn values(&self, name: &[u8]) -> Vec<Vec<u8>> {
        handler::header_values(buffer(), self.0.0, name)
    }
    pub fn set(&self, name: &[u8], value: &[u8]) {
        handler::set_header(self.0.0, name, value);
    }
    pub fn add(&self, name: &[u8], value: &[u8]) {
        handler::add_header_value(self.0.0, name, value);
    }
    pub fn remove(&self, name: &[u8]) {
        handler::remove_header(self.0.0, name);
    }
}
pub struct Body(Type);
impl Body {
    pub fn read(&self) -> Option<Vec<u8>> {
        handler::body(buffer(), self.0.0)
    }
    pub fn write(&self, body: &str) {
        handler::write_body(self.0.0, body);
    }
}
pub struct Request;

impl Request {
    pub fn source_addr(&self) -> Option<Vec<u8>> {
        handler::source_addr(buffer())
    }
    pub fn version(&self) -> Option<Vec<u8>> {
        handler::version(buffer())
    }
    pub fn method(&self) -> Option<Vec<u8>> {
        handler::method(buffer())
    }
    pub fn set_method(&self, method: &[u8]) {
        handler::set_method(method);
    }
    pub fn uri(&self) -> Option<Vec<u8>> {
        handler::uri(buffer())
    }
    pub fn set_uri(&self, uri: &[u8]) {
        handler::set_uri(uri);
    }
    pub fn header(&self) -> Header {
        Header(TYPE_REQUEST)
    }
    pub fn body(&self) -> Body {
        Body(TYPE_RESPONSE)
    }
}
pub struct Response;

impl Response {
    pub fn status_code(&self) -> i32 {
        handler::status_code()
    }
    pub fn set_status_code(&self, code: i32) {
        handler::set_status_code(code);
    }
    pub fn header(&self) -> Header {
        Header(TYPE_RESPONSE)
    }
    pub fn body(&self) -> Body {
        Body(TYPE_RESPONSE)
    }
}
