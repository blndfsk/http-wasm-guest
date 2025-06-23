use http_wasm_guest::{
    Guest,
    host::{Request, Response},
    register,
};

struct Plugin {}

impl Guest for Plugin {
    fn handle_request(&self, request: Request, response: Response) -> (bool, i32) {
        if request.uri().starts_with(b"/.config") {
            response.set_status(403);
            return (false, 0);
        }
        (true, 0)
    }
}
fn main() {
    let plugin = Plugin {};
    register(plugin);
}
