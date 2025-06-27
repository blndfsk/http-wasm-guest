# http-wasm Guest Library

This library implements the [Wasm Guest ABI](https://http-wasm.io/http-handler-abi/), used to interface with 
[http-wasm](https://github.com/http-wasm). 

The main use is for writing traefik-plugins in rust.


- Initial reference code from https://github.com/elisasre/http-wasm-rust/
- API inspired by https://github.com/http-wasm/http-wasm-guest-tinygo


## Usage
Implement the Guest-Trait and register the plugin.

```rust
use http_wasm_guest::{
    Guest,
    host::{Bytes, Request, Response},
    register,
};

struct Plugin;

impl Guest for Plugin {
    fn handle_request(&self, request: Request, _response: Response) -> (bool, i32) {
        request.header().add(&Bytes::from("X-Foo"), &Bytes::from("Bar"));
        (true, 0)
    }
}
fn main() {
    let plugin = Plugin;
    register(plugin);
}
```

## Example
cargo build --target wasm32-wasip1 --examples