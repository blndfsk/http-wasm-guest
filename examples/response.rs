use http_wasm_guest::{
    Guest,
    host::{
        Request, Response,
        feature::{BufferResponse, enable},
    },
    register,
};

struct Plugin;

impl Guest for Plugin {
    fn handle_response(&self, _request: Request, response: Response) {
        response.body().read();
    }
}
fn main() {
    let plugin = Plugin;
    enable(BufferResponse);
    register(plugin);
}
