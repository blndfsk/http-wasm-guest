# http-wasm Guest Library
[![Build](https://github.com/blndfsk/http-wasm-guest/actions/workflows/build.yml/badge.svg)](https://github.com/blndfsk/http-wasm-guest/actions/workflows/build.yml)

This library implements the [Wasm Guest ABI](https://http-wasm.io/http-handler-abi/), used to interface with 
[http-wasm](https://github.com/http-wasm). 

The main use is for writing traefik-plugins in rust.

- Initial reference code from https://github.com/elisasre/http-wasm-rust/
- API inspired by https://github.com/http-wasm/http-wasm-guest-tinygo

## Usage

Implement the 'Guest'-Trait and register the plugin. 

```rust
use http_wasm_guest::{Guest, Request, Response, register};

/// A simple plugin that adds a custom header to each request.
struct Plugin;

impl Guest for Plugin {
    fn handle_request(&self, request: &Request, _response: &Response) -> (bool, i32) {
        let header = request.header();
        header.add(b"X-Bar", b"bar");
        (true, 0)
    }
}

/// Registers the plugin with the http-wasm runtime.
fn main() {
    let plugin = Plugin;
    register(plugin);
}
```
### Tests
cargo test --lib -- --test-threads=1

### Examples
cargo build --target wasm32-wasip1 --examples
