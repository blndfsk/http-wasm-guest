use std::{ops::BitOr, sync::OnceLock};

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

pub struct Feature(i32);
impl BitOr for Feature {
    type Output = Feature;

    fn bitor(self, rhs: Self) -> Feature {
        Feature(self.0 | rhs.0)
    }
}

pub trait Guest {
    fn handle_request(&self, request: Request, response: Response) -> (bool, i32);
    fn handle_response(&self, request: Request, response: Response);
}

pub fn register<T: Guest + 'static>(guest: T) {
    GUEST.get_or_init(|| Handler {
        guest: Box::new(guest),
    });
}

#[unsafe(export_name = "handle_request")]
fn http_request() -> i64 {
    let (next, ctx_next) = match GUEST.get() {
        Some(handler) => handler.guest.handle_request(Request {}, Response {}),
        None => (true, 0),
    };

    if next { (ctx_next as i64) << 32 | 1 } else { 0 }
}

#[unsafe(export_name = "handle_response")]
fn http_response(_req_ctx: i32, _is_error: i32) {
    if let Some(handler) = GUEST.get() {
        handler.guest.handle_response(Request {}, Response {})
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_value() {
        assert_eq!((0 as i64) << 32 | 0, 0);
        assert_eq!((0 as i64) << 32 | 1, 1);
        assert_eq!((16 as i64) << 32 | 1, 68719476737);
        assert_eq!((16 as i64) << 32 | 0, 68719476736);
    }
}
