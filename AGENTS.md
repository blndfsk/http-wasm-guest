# AGENTS.md

## Key Commands

```bash
# Lint and format
cargo fmt --all --check
cargo cl  # alias: clippy --all --all-features

# Test (includes doctests)
cargo nt  # alias: nextest run --lib --all-features --no-fail-fast
cargo test --lib
cargo test --lib --release
cargo test --doc

# Build WASM (required target)
rustup target add wasm32-wasip1
cargo build --target wasm32-wasip1 --example <name>
```

## Project Structure

- Single crate: `http-wasm-guest` (Rust 2024, requires 1.85.1+)
- Entry point: `src/lib.rs` (`Guest` trait + `register` function)
- Host interface: `src/host/` (Request, Response, Headers, Body, logging)
- Examples: `examples/` (run via `./run.sh <example_name>`)

## Design Goals

- Thin wrapper around host functions (minimal abstraction)
- Minimize heap allocations by reusing a static 2048-byte buffer (`src/memory.rs`)
- Returned data (via `to_boxed_slice`) must be heap-allocated for owned types
- Max buffer size: 16MB; log messages truncated at 2048 bytes
- Minimal runtime dependency: `log` crate (optional via `--no-default-features`)

## Build Requirements

- **Target**: Always use `wasm32-wasip1`, not `wasm32-unknown-unknown`
- **Feature**: `log` is enabled by default (`--no-default-features` to disable)

## Testing

- Tests are embedded in `src/lib.rs` under `#[cfg(test)]`
- Run individual tests: `cargo test --lib <test_name>`
- Miri runs on nightly: `cargo +nightly miri test --lib`

## Examples

```bash
./run.sh header   # builds and runs "header" example with Traefik
./run.sh info     # builds and runs "info" example
```

Requires: Podman, Buildah, `traefik:v3.6` and `traefik/whoami` images.

## API Reference

- **Bytes**: `Box<[u8]>` wrapper with `Deref`, `PartialEq`, `.to_str()` (zero-copy UTF-8)
- **Header**: `.get(name)` → first value; `.values_iter(name)` → all values; `.add(name, value)` appends; `.set(name, value)` replaces
- **Body**: No streaming; entire body read/written as buffer
- **Logging**: `log::info!()` etc., respects host level filtering

## Quirks

- Preallocated buffer to avoid heap allocations
- Log messages truncated at 2048 bytes
- Test code uses thread-local storage (different from WASM build)
