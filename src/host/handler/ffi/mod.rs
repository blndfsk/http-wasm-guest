//! FFI declarations for http-wasm host functions.
//!
//! This module provides the interface to the WebAssembly host runtime.
//! In production (WASM target), these are external functions provided by the host.
//! In tests, these are mock implementations that simulate host behavior.

// =============================================================================
// Production FFI - External host functions (WASM target)
// =============================================================================
#[cfg(not(test))]
#[rustfmt::skip]
#[link(wasm_import_module = "http_handler")]
unsafe extern "C" {
    pub(crate) unsafe fn log(level: i32, buf: *const u8, len: i32);
    pub(crate) unsafe fn log_enabled(level: i32) -> i32;
    pub(crate) unsafe fn get_config(buf: *mut u8, buf_limit: i32) -> i32;
    pub(crate) unsafe fn get_method(buf: *mut u8, buf_limit: i32) -> i32;
    pub(crate) unsafe fn set_method(method: *const u8, len: i32);
    pub(crate) unsafe fn get_uri(buf: *mut u8, buf_limit: i32) -> i32;
    pub(crate) unsafe fn set_uri(uri: *const u8, len: i32);
    pub(crate) unsafe fn get_protocol_version(buf: *mut u8, buf_limit: i32) -> i32;
    pub(crate) unsafe fn add_header_value(kind: i32, name: *const u8, name_len: i32, value: *const u8, value_len: i32);
    pub(crate) unsafe fn set_header_value(kind: i32, name: *const u8, name_len: i32, value: *const u8, value_len: i32);
    pub(crate) unsafe fn remove_header(kind: i32, name: *const u8, len: i32);
    pub(crate) unsafe fn get_header_names(kind: i32, buf: *mut u8, buf_limit: i32) -> i64;
    pub(crate) unsafe fn get_header_values(kind: i32, name: *const u8, len: i32, buf: *mut u8, buf_limit: i32) -> i64;
    pub(crate) unsafe fn read_body(kind: i32, buf: *mut u8, buf_limit: i32) -> i64;
    pub(crate) unsafe fn write_body(kind: i32, body: *const u8, len: i32);
    pub(crate) unsafe fn get_status_code() -> i32;
    pub(crate) unsafe fn set_status_code(code: i32);
    pub(crate) unsafe fn enable_features(feature: i32) -> i32;
    pub(crate) unsafe fn get_source_addr(buf: *mut u8, buf_limit: i32) -> i32;
}

// =============================================================================
// Test FFI - Mock implementations
// =============================================================================
#[cfg(test)]
pub(crate) mod mock;

// Re-export mock functions with the same names as the extern declarations
#[cfg(test)]
pub(crate) use mock::*;
