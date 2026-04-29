## v0.11.3

### Features
- **New `PartialEq` trait implementations for `Bytes`**: Enables comparisons between `Bytes` and byte slices, arrays, and strings:
  - `Bytes` <-> `[u8]` and `&[u8]`
  - `Bytes` <-> `[u8; N]` and `&[u8; N]` for any `N`
  - `Bytes` <-> `&Bytes`
  - `Bytes` <-> `str` and `&str`
  - All combinations work bidirectionally: `slice == bytes`, `bytes == slice`, `arr == bytes`, `str == bytes`, etc.

### Refactoring
- Removed unsafe implementation of `Borrow<[u8; N]>` for `Bytes`. 

### Safety & Lints
- Added strict lint rules in `Cargo.toml`:
  - `panic = "deny"`, `unwrap_used = "deny"` — enforce safe error handling
  - `map_unwrap_or = "warn"`, `get_unwrap = "warn"`, `unwrap_in_result = "warn"` — warn against common unsafe patterns
  - `string_slice = "warn"` — warn on string-slice conversions


## v0.11.2

### API Changes
- `.header()` and `.body()` can now be accessed as fields (`.header`, `.body`)

### Bug Fixes
- Safeguard against ffi::read_body returning size 0 without EOF marker
- Added safeguard against host responses exceeding 16MB

### Improvements
- Inlined FFI conversion helpers for performance
- Enhanced `Body::read()` documentation with feature flag requirements
- Added comprehensive project documentation (Copilot instructions)

### Testing
- Added tests for empty body and oversized response edge cases
- Updated examples to use new field access syntax

## v0.11.1

### Features
- **New Methods:**
  - `names_iter()` - Returns an iterator over all header names without allocation
  - `values_iter(name)` - Returns an iterator over values for a specific header name without allocation  
  - `entries_iter()` - Returns an iterator over all headers as name-value pairs without allocation

## v0.11.0

### API-Breaking Changes

- rename Header::values to entries
- rename Header::get_all to values
- remove unchecked UTF-8 conversion from Bytes

### Features

- add traits Copy, Clone and BitOrAssign for Feature flags

### Refactoring

- support parallel test execution
- safe conversions between i32 and usize

## v0.10.3

### Bug Fixes
- Fix infinite loop in host logger setup. 

### Testing & Safety
- Add test for body max size limit and mock oversized body in FFI.

### CI/CD
- Ignore docs and markdown files in CI triggers.

## v0.10.2

No API changes. This release focuses on robustness and correctness of the internal FFI layer and memory handling.

### Bug Fixes
- Fix truncation in `request.uri()` when host data exceeds 2048 bytes.

### Safety
- Harden against malformed FFI return values.
- Limit `request.body()` and `response.body()` reads to 16 MB.

## v0.10.1

### API-Breaking Changes

**Logger Integration Refactor:**  
  The logging API has been restructured. The host logger implementation is now provided only if the feature flag `log` is enabled.
  - To disable logging integration, set `default-features = false` in your dependency declaration.
  - The low-level logging functions (`write`, `enabled`) are now accessed via `host::log`.
  - The logger initialization functions are now `HostLogger::init()` and `HostLogger::init_with_level()`.

**Removed `Default` Implementation for `Request` and `Response`:**
  The `Default` trait is no longer implemented for the `Request` and `Response` types and the `new()` functions are no longer visible.
  This change was made to prevent construction of these objects, and to clarify the intended usage of these types.

### Bug Fixes
- `init_with_level(level: Level)` now determines the configured level of the host correctly.

### Refactoring & Improvements
- Simplified header buffer handling in `host/handler`.
- Prevented unnecessary copying in header handling.

### Documentation & Usability
- Expanded the README with:
  - A detailed "Testing" section, including prerequisites for running `run.sh` (Podman, Buildah, Rust WASM target, container images).
  - Step-by-step instructions for running and interpreting example plugins.
  - A troubleshooting section for common WASM build and runtime issues.
- General documentation improvements.
- Added Miri badge to the README.

## v0.9.3

### Testing
- Add comprehensive unit tests for all host modules
- Add tests for header duplicates, logger, and response handling
- Add GitHub Actions workflow for Miri testing

### Bug Fixes (discovered via Miri)
- **Fix memory leak from incorrect `mem::forget` usage** - Removed improper use of `std::mem::forget` on Vec buffers in `header_names()` and `header_values()` functions. This bug was triggered when HTTP headers exceeded the default 2048-byte buffer size, causing the overflow path to allocate a larger buffer that was never deallocated. Every request with large headers would leak memory.
- **Fix potential undefined behavior in buffer overflow handling** - The overflow code path was incorrectly using the raw i64 return value as the buffer length, instead of properly splitting it into count and length components using `split_i64()`. This was triggered when headers exceeded the 2048-byte buffer, causing the code to interpret a large combined value as the slice length, leading to reads from uninitialized memory.

### CI/CD
- Add code coverage workflow with Codecov integration
- Simplify and improve CI workflow jobs

### Maintenance
- Code formatting (rust fmt)
- Documentation and formatting improvements

## v0.9.2
### New
- Add header and info example configs
- provide script for easy test-deployment of examples

### API-Change
- `header.values` and `header.get` renamed to better reflect returned values
- feature flags moved to the admin package
- revert trait based abstractions
- expose req_ctx and isError to response-handler

## v0.8.0
API-Change: public API refactor to use trait-based request/response/body/header abstractions.
Documentation: refreshed public API docs and feature flag descriptions.
Formatting: codebase formatted.

## v0.7.0
API-Change: get_config() renamed to config(), returns Bytes instead of Result<String, FromUtf8Error>

## v0.6.2
Documentation

## v0.6.1
all setters now accept the more generic type ```&[u8]```.
 ```rust
let header = request.header();
header.add(&Bytes::from("X-Foo"), &Bytes::from("foo"));
header.add(b"X-Bar", b"bar"); // this is now possible
```

## v0.6.0
### New
#### Header
- provide new method ```pub fn get(&self) -> HashMap<Bytes, Vec<Bytes>>``` which returns a map of header names together with the corresponding values

### API Change
#### Bytes
- as_str renamed to to_str to better adhere to rust naming standards
- to_str now returns a Result<&str, Utf8Error> to be able to react to non-utf8 input

## v0.5.1
### New
Ability to construct Bytes from &[u8]
```rust
let b = Bytes::from(b"test");
```

## v0.5.0
### New
Change own log macros to the standard log-crate
