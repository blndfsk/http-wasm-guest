## v0.9.4

### Bug Fixes
- `init_with_level(level: Level)` now determines the configured level of the host correctly.

### Refactoring & Improvements
- Added `new()` constructors to `Request` and `Response`.
- Simplified header buffer handling in `host/handler`.
- Prevented unnecessary copying in header handling.
- Added Miri badge to the README.

### Documentation & Usability
- Expanded the README with:
  - A detailed "Testing" section, including prerequisites for running `run.sh` (Podman, Buildah, Rust WASM target, container images).
  - Step-by-step instructions for running and interpreting example plugins.
  - A troubleshooting section for common WASM build and runtime issues.
- Clarified plugin compatibility between Traefik and Envoy (http-wasm ABI).
- General documentation improvements for onboarding and developer experience.

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
