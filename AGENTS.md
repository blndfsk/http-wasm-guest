# AGENTS.md

Human-facing documentation (design goals, memory model, usage, examples, troubleshooting) lives in [README.md](README.md); per-method cost details live in the documentation of the public functions. Read the README before making changes.

## Project structure

- Single crate: `http-wasm-guest` (Rust 2024, MSRV 1.85.1)
- `src/lib.rs` — `Guest` trait, `register`, and the WASM entry points
- `src/host/` — host handles: `request`, `response`, `header`, `body`, `admin`, `feature`, `log`
- `src/host/handler/` — FFI bindings and read logic (shared-buffer reads, `MAX_ALLOC_SIZE` fallback)
- `src/memory.rs` — the shared 2048-byte read buffer (`with_buffer`)
- `src/host_logger.rs` — `HostLogger` behind the `log` feature
- `examples/` — `header` and `info` (run via `./run.sh <example_name>`)

## Key commands

```bash
# Lint and format
cargo fmt --all --check
cargo cl  # alias: clippy --all --all-features

# Test (includes doctests)
cargo nt  # alias: nextest run --lib --all-features --no-fail-fast
cargo test --lib
cargo test --lib --release
cargo test --doc
cargo test --lib <test_name>   # run a single test

# Coverage
cargo llvm-cov --quiet --lib --show-missing-lines

# Miri (nightly)
cargo +nightly miri test --lib

# Build WASM (required target)
rustup target add wasm32-wasip1
cargo build --target wasm32-wasip1 --example <name>
```

The `nt` and `cl` aliases are defined in `.cargo/config.toml`.

## Build requirements

- **Target**: always use `wasm32-wasip1`, not `wasm32-unknown-unknown`
- **Features**: `log` is enabled by default; disable it with `--no-default-features`
- Runtime dependencies: `bytes` plus optional `log` — keep additions minimal and WASM-compatible

## Internal memory model

- Every host read is written into a single shared 2048-byte buffer (`src/memory.rs`, accessed via `with_buffer`) that is reused for every host call
- Fields that do not fit fall back to one extra host call into an exactly-sized heap allocation, capped at `MAX_ALLOC_SIZE = 0xFFFFFF` (just under 16 MiB) in `src/host/handler/mod.rs`; larger reads are truncated
- `body` reads (`read`) loop in ≤2048-byte chunks (one host call per chunk) until EOF or the cap is reached
- `body` streaming (`read_iter`) issues one `read_body` host call per chunk without accumulating, ending on EOF or a zero-length read — no cap applies since nothing is retained
- No API returns a zero-copy view of host data — reads always return owned copies (`Bytes::from(Box<[u8]>)`)
- Log messages routed through the `log` feature are formatted into the same 2048-byte buffer and truncated there by `HostLogger` (guest-side, not the host); raw `host::log::write` passes the slice straight to the host with no guest-side truncation

## API behavior notes

- **Headers**: names come back lowercase; lookups are case-insensitive. Every read = one host call + owned copies (one heap allocation per name/value). `.get(name)` returns the first value but still performs a full lookup and allocates all values. `set`/`add`/`remove` trap if the host fails
- **Body**: `read` drains in ≤2048-byte chunks until EOF (capped at `MAX_ALLOC_SIZE`), returning owned `Bytes`; `read_iter` streams chunks of at most 2048 bytes without accumulating; `write` is stateful — the first call replaces the body, later calls append
- **Response**: `.status()` may panic when called before `handle_response`
- **Features**: `admin::enable(flags)` returns the full bitflag of features the host supports; enable before returning from `handle_request` (or during init to fail fast)

## Testing notes

- Unit tests are embedded next to the code in every module under `#[cfg(test)]`, not only in `src/lib.rs`
- Test builds use a thread-local copy of the shared buffer (`src/memory.rs`, `#[cfg(test)]`) instead of the single static buffer used in WASM builds, so tests can run in parallel
