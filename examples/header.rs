use http_wasm_guest::{
    Guest,
    host::{Bytes, Request, Response},
    register,
};

struct Plugin {}

impl Guest for Plugin {
    fn handle_request(&self, request: Request, _response: Response) -> (bool, i32) {
        request
            .header()
            .add(&Bytes::from("X-Foo"), &Bytes::from("Bar"));
        (true, 0)
    }
}
fn main() {
    let plugin = Plugin {};
    register(plugin);
}
