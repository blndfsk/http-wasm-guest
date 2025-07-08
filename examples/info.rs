use http_wasm_guest::{
    Guest,
    host::{Request, Response},
    info, register,
};

struct Plugin;

impl Guest for Plugin {
    fn handle_request(&self, request: Request, _response: Response) -> (bool, i32) {
        info!("{} {}", request.method(), request.uri());
        (true, 0)
    }
}
fn main() {
    let plugin = Plugin;
    register(plugin);
}
