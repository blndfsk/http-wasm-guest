use http_wasm_guest::{
    Guest,
    host::{Bytes, Request, Response},
    register,
};

struct Plugin;

impl Guest for Plugin {
    fn handle_request(&self, request: Request, _response: Response) -> (bool, i32) {
        let header = request.header();
        header.add(&Bytes::from("X-Foo"), &Bytes::from("foo"));
        header.add(b"X-Bar", b"bar");
        (true, 0)
    }
}
fn main() {
    let plugin = Plugin;
    register(plugin);
}
