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

# Coverage
cargo llvm-cov --quiet --lib --show-missing-lines

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
- Minimize heap allocations by reusing a static 2048-byte buffer (`src/memory.rs`): no allocation when a field fits, one extra host call against an exactly-sized heap allocation otherwise
- Returned data must be owned copies: the shared buffer is reused for every subsequent host call, so zero-copy views are impossible; `Bytes::from(Box<[u8]>)` moves (no extra copy)
- Max field size: just under 16 MiB (`MAX_ALLOC_SIZE = 0xFFFFFF`); log messages truncated at 2048 bytes by `HostLogger` (guest-side, not the host)
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

Per-method cost notes live in the rustdoc of `src/host/`, which follows the [http-wasm HTTP Handler ABI](https://http-wasm.io/http-handler-abi/).

- **Bytes**: re-export of `bytes::Bytes` (refcounted, heap-backed slice; `Deref` to `[u8]`, `PartialEq` with `str`/`&[u8]`, zero-copy UTF-8 via `std::str::from_utf8`; there is **no** `.to_str()` method in bytes 1.x)
- **Header**: names come back lowercase; lookups are case-insensitive. Every read = one host call + owned copies (one heap allocation per name/value). `.get(name)` returns the first value but still performs a full lookup and allocates all values. `set`/`add`/`remove` trap if the host fails
- **Body**: reads drain in ≤2048-byte chunks (one host call per chunk) until EOF, returning owned `Bytes`; `write` is stateful — first call replaces the body, later calls append
- **Response**: `.status()` may panic when called before `handle_response`
- **Admin/features**: `admin::enable(flags)` returns the full bitflag of features the host supports; enable before returning from `handle_request` (or during init to fail fast)
- **Logging**: `log::info!()` etc., respects host level filtering; raw `host::log::write` passes the slice straight to the host with no guest-side truncation

## Quirks

- Static 2048-byte buffer receives every host read; oversized fields fall back to an exactly-sized heap allocation (max just under 16 MiB)
- No API returns a zero-copy view of host data — results are always owned copies
- Log truncation at 2048 bytes is done by `HostLogger` in guest memory, not by the host
- Test code uses thread-local storage (different from WASM build)
