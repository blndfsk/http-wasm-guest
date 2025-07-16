use http_wasm_guest::{
    Guest,
    host::{self, Request, Response, feature::BufferRequest},
    register,
};
use log::info;

struct Plugin;

impl Guest for Plugin {
    fn handle_request(&self, request: Request, _response: Response) -> (bool, i32) {
        info!("{} {}", request.method(), request.uri());
        info!("{:?}", request.header().get());
        (true, 0)
    }
}
fn main() {
    host::log::init().expect("error initializing logger");
    host::feature::enable(BufferRequest);
    let plugin = Plugin;
    register(plugin);
}
