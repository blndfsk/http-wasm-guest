# Copilot Instructions for http-wasm-guest

## Project Overview

**http-wasm-guest** is a thin, high-performance Rust library providing the guest-side API for the [Wasm Guest ABI](https://http-wasm.io/http-handler-abi/). It enables developers to write HTTP middleware plugins in Rust that run on http-wasm-compatible runtimes (particularly Traefik). The library is **not opinionated**: values from the host are passed through without manipulation, allowing plugins to implement any use-case. The only exception is memory handling: the maximum allocation is limited to prevent runaway memory usage in constrained WASM environments.

## Architecture

The library is structured into two main layers:

1. **Public API** (`src/lib.rs`) - The `Guest` trait that plugins implement:
   - `handle_request(request, response)` → called before request processing
   - `handle_response(req_ctx, request, response, is_error)` → called after upstream processing
   - `register(plugin)` → entry point to wire up the plugin

2. **Host Module** (`src/host/`) - Low-level bindings and abstractions for interacting with the WASM host:
   - `Request` / `Response` - High-level handles for HTTP messages
   - `Header` - Header collection with iteration and access patterns
   - `Body` - Body reading and writing 
   - `Bytes` - Low-level byte buffer abstraction (zero-copy when possible)
   - `Log` - host logging interface, a `HostLogger` provides standard log-functionality for the guest
   - `Feature` - Runtime feature flags for capabilities like trailers and body buffering
   - `Admin` - Configuration interface from host
   - `handler/mod.rs` - Raw FFI bindings to host functions (internal)

**Key Design Pattern:** The library uses preallocated stack buffers for all read operation and logging, keeping those hot paths efficient. 

## Build and Test

### Targets

```bash
# Standard library tests (123 unit tests)
cargo test --lib

# Single test module
cargo test --lib host::bytes::

# Single test
cargo test --lib bytes_equality

# Format check
cargo fmt --check

# Build for WASM
cargo build --target wasm32-wasip1
cargo build --release --target wasm32-wasip1
```

### Running Examples

The `examples/` directory contains two working plugins (header, info). They cannot run as native binaries (missing host symbols), but can be tested via `run.sh`:

```bash
./run.sh header      # Build header example and run in Traefik
./run.sh info        # Build info example and run in Traefik
```

Prerequisites: Podman, buildah, `wasm32-wasip1` target, network access to pull images.

## Key Conventions

- **WASM Memory Constraint:** A major constraint of wasm32-wasip1 is available memory. Unnecessary copies and allocations must be prevented. The library design prioritizes zero-copy patterns and preallocated buffers for this reason.

- **Not Opinionated:** Host values are passed through without manipulation, enabling plugins to implement any use-case. 

- **Memory Model:**
  - Preallocated fixed-size buffer used for read operations and formatting log messages
  - In the overflow path, the maximum allocated memory is limited to 16MB

- **Bytes Abstraction:** The custom `Bytes` type is a thin wrapper around `Box<[u8]>` that provides:
  - `Deref<Target=[u8]>` for transparent slice access
  - `PartialEq` against `str` and `&[u8]` for ergonomic comparisons
  - Zero-copy UTF-8 validation via `.to_str()`
  - Efficient conversions from `Vec`, `Box<[u8]>`, and byte slices

- **Header API:** Headers support lookup and multi-value retrieval:
  - `.get(name)` → first value or None
  - `.values_iter(name)` → iterator over all values
  - `.add(name, value)` → append header
  - `.set_header(name, value)` replaces all values

- **Body Handling:** `Body` used for requests and responses (no streaming):
  - Both request and response bodies are read/written as complete buffers
  - No chunked or streaming API; entire body must fit in memory

- **Error Handling:** Debug assertions detect protocol errors—cases where the host reports values not in line with the http-wasm specification (e.g., buffer size mismatches). These should never occur with a conforming host and indicate a host implementation bug, not a library defect.

- **Logging:** Log messages formatted into fixed buffer via `log!` macro. Level filtering respects host configuration. Use `log::trace!` for hot-path debugging if needed.

## Dependencies

### Runtime (always included)
- **log** (0.4, optional via feature flag): Structured logging with host backend

### Dev
- Test infrastructure is built-in; no external test framework

## Examples

Both examples implement `Guest` and demonstrate request manipulation:
- `header.rs` - Adds custom headers to requests
- `info.rs` - Logs request info via the `log` crate

See `examples/` directory and corresponding `.yml` config files for Traefik setup.
