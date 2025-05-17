# http-wasm Guest Library

[http-wasm](https://github.com/http-wasm) is HTTP client middleware implemented in WebAssembly. This is a library that implements the [Guest ABI](https://http-wasm.io/http-handler-abi/).


Initial reference code from https://github.com/elisasre/http-wasm-rust/
API inspired by https://github.com/http-wasm/http-wasm-guest-tinygo


## Usage
Implement the Guest-Trait and register the plugin.

## Example

```rust
use http_wasm_guest::{
    feature::{enable_feature, Feature},
    host::{get_config, Request, Response},
    info, register, Guest,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    pub rules: Vec<String>,
}

struct Plugin {
}

impl Guest for Plugin {
    fn handle_request(&self, request: Request, _response: Response) -> (bool, i32) {
        info!("uri: {}", to_str(request.uri().as_deref()));
        (true, 0)
    }
}
fn main() {
    let config: Config = get_config()
        .and_then(|v| serde_json::from_slice(&v).ok())
        .unwrap();
    info!("rules {:?}", &config.rules);
    enable_feature(Feature::BufferRequest);

    register(plugin);
}

fn to_str(vec: Option<&[u8]>) -> String {
    vec.and_then(|v| std::str::from_utf8(v).ok())
        .unwrap_or_default()
        .to_string()
}
```