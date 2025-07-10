use http_wasm_guest::{
    Guest,
    host::{
        Bytes, Request, Response,
        feature::{self, BufferResponse},
    },
    register,
};

struct Plugin;

impl Guest for Plugin {
    fn handle_response(&self, _request: Request, response: Response) {
        response.body().write(&Bytes::from(b"test".as_slice()));
    }
}
fn main() {
    let plugin = Plugin;
    feature::enable(BufferResponse);
    register(plugin);
}
