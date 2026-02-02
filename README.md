# http-wasm Guest Library
[![Build](https://github.com/blndfsk/http-wasm-guest/actions/workflows/build.yml/badge.svg)](https://github.com/blndfsk/http-wasm-guest/actions/workflows/build.yml)

This library implements the [Wasm Guest ABI](https://http-wasm.io/http-handler-abi/), used to interface with 
[http-wasm](https://github.com/http-wasm). 

The main use is for writing traefik-plugins in rust.

- Initial reference code from https://github.com/elisasre/http-wasm-rust/
- API inspired by https://github.com/http-wasm/http-wasm-guest-tinygo

## Building
This is a library for creating wasm-plugins and is not useful standalone. 

### Tests
cargo test --lib

### Examples
cargo build --target wasm32-wasip1 --examples
