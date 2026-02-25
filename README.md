# http-wasm Guest Library

[![crate](https://img.shields.io/crates/v/http-wasm-guest.svg)](https://crates.io/crates/http-wasm-guest)
[![Test](https://github.com/blndfsk/http-wasm-guest/actions/workflows/test.yml/badge.svg)](https://github.com/blndfsk/http-wasm-guest/actions/workflows/test.yml)
[![Miri](https://github.com/blndfsk/http-wasm-guest/actions/workflows/miri.yml/badge.svg)](https://github.com/blndfsk/http-wasm-guest/actions/workflows/miri.yml)
[![codecov](https://codecov.io/github/blndfsk/http-wasm-guest/graph/badge.svg)](https://codecov.io/github/blndfsk/http-wasm-guest)

This library provides a Rust implementation for the [Wasm Guest ABI](https://http-wasm.io/http-handler-abi/) and interfaces with
[http-wasm](https://github.com/http-wasm).

It is designed for writing Traefik plugins in Rust, and works with any http-wasm compatible runtime.

## Design Goals

- Not opinionated, the focus is to provide a very thin wrapper around the host functions.
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

### Test

#### Prerequisites

To run the examples using the `run.sh` script, you will need the following tools and resources installed on your system:

- **[Podman](https://podman.io/):** Used for running rootless containers and pods (a drop-in replacement for Docker).
- **[Buildah](https://buildah.io/):** Used for building container images.
- **Rust toolchain:** With the `wasm32-wasip1` target installed.
- **Network access:** To pull the `traefik:v3.6` and `traefik/whoami` container images if not already present locally.
- **Sufficient permissions:** To run containerized workloads (may require appropriate user group membership).

You can install Podman and Buildah using your system's package manager. For example, on Ubuntu:

```shell
sudo apt update
sudo apt install podman buildah
```

Make sure you have the WASM target for Rust:

```shell
rustup target add wasm32-wasip1
```

#### Running the Example

You can run the examples via the provided `run.sh` script. This creates a running container for the traefik-server with the plugin configured and the whois-service wired into the router.

```shell
$ ./run.sh header
[lots of logging output]
```

#### Interpreting Example Output

After running the example, you can test the plugin by sending a request to the local server:

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

Look for the presence of the `X-Custom-Header: FooBar` line in the output. This indicates that your plugin is running and modifying the request as expected. You can modify and re-run the examples to experiment with different plugin behaviors.

---

### Troubleshooting

#### Common Issues When Building for WASM

- **Missing WASM Target:**  
  If you see errors about unknown target or missing standard library, make sure you have added the WASM target:
  ```shell
  rustup target add wasm32-wasip1
  ```

- **Build Fails with Linking Errors:**  
  Ensure you are using the correct target triple (`wasm32-wasip1`) and not `wasm32-unknown-unknown` or others.

- **Plugin Not Loaded or No Effect:**  
  - Double-check that your `.wasm` file is being mounted and referenced correctly in your server/proxy configuration.
  - Review logs for errors about plugin loading or execution.

- **Crate Features or Dependency Issues:**  
  Some crates do not support WASM targets. Keep dependencies minimal and check for WASM compatibility.

- **Debugging WASM Plugins:**  
  Use logging (`log` crate) to emit messages from your plugin. Ensure the host runtime is configured to display or capture logs.
