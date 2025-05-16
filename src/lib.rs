use std::sync::OnceLock;

use request::Request;
use response::Response;

pub mod host;
pub mod log;
pub mod request;
pub mod response;

struct Handler {
    guest: Box<dyn Guest>,
}
unsafe impl Send for Handler {}
unsafe impl Sync for Handler {}

static GUEST: OnceLock<Handler> = OnceLock::new();

pub trait Guest {
    fn handle_request(&self, request: Request, response: Response) -> bool;
    fn handle_response(&self, request: Request, response: Response);
}

pub fn register<T: Guest + 'static>(guest: T) {
    GUEST.get_or_init(|| Handler {
        guest: Box::new(guest),
    });
}

#[unsafe(export_name = "handle_request")]
pub fn http_request() -> u64 {
    let rc = match GUEST.get() {
        Some(handler) => handler.guest.handle_request(Request {}, Response {}),
        None => true,
    };

    if rc { 1 } else { 0 }
}

#[unsafe(export_name = "handle_response")]
fn http_response(_req_ctx: i32, _is_error: i32) {
    if let Some(handler) = GUEST.get() {
        handler.guest.handle_response(Request {}, Response {})
    };
}
