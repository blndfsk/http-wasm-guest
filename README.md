# http-wasm-guest

Implementation of the ABI described in https://http-wasm.io/http-handler-abi/

Initial reference code from https://github.com/elisasre/http-wasm-rust/



## Usage
```rust
use http_wasm_guest::{host::get_config, register, request::Request, response::Response, Guest};
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    pub rules: Vec<String>,
}

struct Plugin {}

impl Guest for Plugin {
    fn handle_request(&self, _request: Request, _response: Response) -> bool {
        true
    }

    fn handle_response(&self, _request: Request, _response: Response) {}
}

fn main() {
    let _config: Option<Config> =
        get_config().and_then(|s| serde_json::from_slice(&s).ok());
    let plugin = Plugin {};

    register(plugin);
}
```