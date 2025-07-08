//! library implementing the [Guest ABI](https://http-wasm.io/http-handler-abi/) for interfacing with
//! [http-wasm](https://github.com/http-wasm)
use std::sync::OnceLock;

use host::{Request, Response};

pub mod host;

struct Handler {
    guest: Box<dyn Guest>,
}
unsafe impl Send for Handler {}
unsafe impl Sync for Handler {}

static GUEST: OnceLock<Handler> = OnceLock::new();

pub trait Guest {
    fn handle_request(&self, _request: Request, _response: Response) -> (bool, i32) {
        (true, 0)
    }
    fn handle_response(&self, _request: Request, _response: Response) {}
}

pub fn register<T: Guest + 'static>(guest: T) {
    GUEST.get_or_init(|| Handler {
        guest: Box::new(guest),
    });
}

#[unsafe(export_name = "handle_request")]
fn http_request() -> i64 {
    let (next, ctx_next) = match GUEST.get() {
        Some(handler) => handler
            .guest
            .handle_request(Request::new(), Response::new()),
        None => (true, 0),
    };

    if next { (ctx_next as i64) << 32 | 1 } else { 0 }
}

#[unsafe(export_name = "handle_response")]
fn http_response(_req_ctx: i32, _is_error: i32) {
    if let Some(handler) = GUEST.get() {
        handler
            .guest
            .handle_response(Request::new(), Response::new())
    };
}
