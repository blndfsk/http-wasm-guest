# http-wasm Guest Library

[http-wasm](https://github.com/http-wasm) is HTTP client middleware implemented in WebAssembly. This is a library that implements the [Guest ABI](https://http-wasm.io/http-handler-abi/).


Initial reference code from https://github.com/elisasre/http-wasm-rust/
API inspired by https://github.com/http-wasm/http-wasm-guest-tinygo


## Usage
Implement the Guest-Trait and register the plugin.

## Example
cargo build --target wasm32-wasip1 --examples