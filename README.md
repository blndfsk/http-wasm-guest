# http-wasm Guest Library

This library provides a Rust implementation for the [Wasm Guest ABI](https://http-wasm.io/http-handler-abi/) and interfaces with
[http-wasm](https://github.com/http-wasm).

It is designed for writing Traefik plugins in Rust, and works with any http-wasm compatible runtime.

## Design Goals

- Minimal dependency footprint: only the `log` crate is required at runtime.
- Low-level `Byte` abstraction to enable all use-cases.
- Memory-efficient data handling to suit constrained Wasm environments.

## Credits

- Initial reference code from [http-wasm-rust](https://github.com/elisasre/http-wasm-rust/)
- API inspired by [http-wasm-guest-tinygo](https://github.com/http-wasm/http-wasm-guest-tinygo)

## Usage

Add the dependency to your project:

`cargo add http-wasm-guest`

Implement the `Guest` trait and register the plugin. See the [examples](/examples) for complete code.

```rust
use http_wasm_guest::{Guest, Request, Response, register};

/// A minimal plugin that adds a custom header to each request.
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

### Build

Add the WASI target for building the plugin:

`rustup target add wasm32-wasip1`

Build the plugin with

`cargo build --target wasm32-wasip1 --release`

### Deploy

[Install your plugin](https://plugins.traefik.io/install)
```
plugins/
└── src
    └── plugindemowasm
        └── plugin.wasm
```
