# http-wasm Guest Library

[![crate](https://img.shields.io/crates/v/http-wasm-guest.svg)](https://crates.io/crates/http-wasm-guest)
[![Test](https://github.com/blndfsk/http-wasm-guest/actions/workflows/test.yml/badge.svg)](https://github.com/blndfsk/http-wasm-guest/actions/workflows/test.yml)

This library provides a Rust implementation for the [Wasm Guest ABI](https://http-wasm.io/http-handler-abi/) and interfaces with
[http-wasm](https://github.com/http-wasm).

It is designed for writing Traefik plugins in Rust, and works with any http-wasm compatible runtime.

## Design Goals

- Not opinionated, the focus is to provide a very thin wrapper around the
  host functions.
- Minimal dependency footprint: only the `log` crate is required at runtime.
- Low-level `Byte` abstraction to enable all use-cases.
- Memory-efficient data handling to suit constrained Wasm environments.

## Credits

- Initial reference code from [http-wasm-rust](https://github.com/elisasre/http-wasm-rust/)
- API inspired by [http-wasm-guest-tinygo](https://github.com/http-wasm/http-wasm-guest-tinygo)

## Usage

Add the dependency to your project:

`cargo add http-wasm-guest`

Implement the `Guest` trait and register the plugin. See the [examples](examples) for complete code.

```rust
use http_wasm_guest::{
    Guest,
    host::{Request, Response},
    register,
};
struct Plugin {}

impl Guest for Plugin {
    fn handle_request(&self, request: &Request, _response: &Response) -> (bool, i32) {
        request.header().add(b"X-Custom-Header", b"FooBar");
        (true, 0)
    }
}

fn main() {
    let plugin = Plugin {};
    register(plugin);
}
```

### Build

Add the WASM target for building the plugin:

`rustup target add wasm32-wasip1`

Build the library with

`cargo build --target wasm32-wasip1 --release`

### Test

You can run the examples via the provided `run.sh` script. This creates a running container for the traefik-server with the plugin configured and the whois-service wired into the router.

```shell
$ ./run.sh header
[lots of logging output]
```

```shell
$ curl  http://whoami.localhost:8080
Hostname: pensive_curran
IP: 127.0.0.1
IP: ::1
RemoteAddr: [::1]:53364
GET / HTTP/1.1
Host: whoami.localhost:8080
User-Agent: curl/8.18.0
Accept: */*
Accept-Encoding: gzip
X-Custom-Header: FooBar
[more output]
```
