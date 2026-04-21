// =============================================================================
// Test FFI - Mock implementations
// =============================================================================

// Re-export mock functions with the same names as the extern declarations
use std::{ptr, slice::from_raw_parts};

use crate::host::handler::test;

/// Copy bytes from source to destination buffer.
/// Returns the number of bytes copied (limited by buf_limit).
fn copy_to_buf(src: &[u8], buf: *mut u8, buf_limit: i32) -> i32 {
    let limit = buf_limit as usize;
    let len = src.len().min(limit);
    unsafe { ptr::copy_nonoverlapping(src.as_ptr(), buf, len) };
    len as i32
}

// -------------------------------------------------------------------------
// Logging
// -------------------------------------------------------------------------

pub(crate) unsafe fn log(_level: i32, _buf: *const u8, _len: i32) {
    // No-op: discard log messages in tests
}

pub(crate) unsafe fn log_enabled(level: i32) -> i32 {
    // Enable info, warn, error, disable debug
    if 0 <= level { 1 } else { 0 }
}

// -------------------------------------------------------------------------
// Configuration
// -------------------------------------------------------------------------

/// Returns config data.
pub(crate) unsafe fn get_config(buf: *mut u8, buf_limit: i32) -> i32 {
    copy_to_buf(br#"{ "config" : "test1"}"#, buf, buf_limit)
}

// -------------------------------------------------------------------------
// Request Method
// -------------------------------------------------------------------------

pub(crate) unsafe fn get_method(buf: *mut u8, buf_limit: i32) -> i32 {
    copy_to_buf(b"GET", buf, buf_limit)
}

pub(crate) unsafe fn set_method(_method: *const u8, _len: i32) {
    // No-op in tests
}

// -------------------------------------------------------------------------
// Request URI
// -------------------------------------------------------------------------

pub(crate) unsafe fn get_uri(buf: *mut u8, buf_limit: i32) -> i32 {
    copy_to_buf(b"https://test", buf, buf_limit)
}

pub(crate) unsafe fn set_uri(_uri: *const u8, _len: i32) {
    // No-op in tests
}

// -------------------------------------------------------------------------
// Protocol Version
// -------------------------------------------------------------------------

pub(crate) unsafe fn get_protocol_version(buf: *mut u8, buf_limit: i32) -> i32 {
    copy_to_buf(b"HTTP/2.0", buf, buf_limit)
}

// -------------------------------------------------------------------------
// Headers
// -------------------------------------------------------------------------

pub(crate) unsafe fn add_header_value(_kind: i32, _name: *const u8, _name_len: i32, _value: *const u8, _value_len: i32) {
    // No-op in tests
}

pub(crate) unsafe fn set_header_value(_kind: i32, _name: *const u8, _name_len: i32, _value: *const u8, _value_len: i32) {
    // No-op in tests
}

pub(crate) unsafe fn remove_header(_kind: i32, _name: *const u8, _len: i32) {
    // No-op in tests
}

/// Returns header names: X-FOO, x-bar, x-baz
/// Return value: count in upper 32 bits, length in lower 32 bits
pub(crate) unsafe fn get_header_names(_kind: i32, buf: *mut u8, buf_limit: i32) -> i64 {
    let data = b"X-FOO\0x-bar\0x-baz\0";
    let len = copy_to_buf(data, buf, buf_limit);
    (3i64 << 32) | (len as i64)
}

/// Returns header values based on name:
/// - X-FOO: ["test1"]
/// - x-bar: ["test2", "test3"]
/// - x-baz: ["test4"]
///
/// Return value: count in upper 32 bits, length in lower 32 bits
pub(crate) unsafe fn get_header_values(_kind: i32, name: *const u8, name_len: i32, buf: *mut u8, buf_limit: i32) -> i64 {
    let name = unsafe { from_raw_parts(name, name_len as usize) };

    match name {
        b"X-FOO" => (1i64 << 32) | copy_to_buf(b"test1\0", buf, buf_limit) as i64,
        b"x-bar" => (2i64 << 32) | copy_to_buf(b"test2\0test3\0", buf, buf_limit) as i64,
        b"x-baz" => (2i64 << 32) | copy_to_buf(b"test4\0test4\0", buf, buf_limit) as i64,
        _ => 0i64,
    }
}

// -------------------------------------------------------------------------
// Body
// -------------------------------------------------------------------------

/// Returns mock body content with EOF flag set.
/// For kind=99, returns a full buffer of data without EOF to simulate an oversized body.
/// Return value: EOF (1) in upper 32 bits, length in lower 32 bits
pub(crate) unsafe fn read_body(kind: i32, buf: *mut u8, buf_limit: i32) -> i64 {
    match kind {
        test::kinds::EMPTY_BODY_WITHOUT_EOF => 0,
        test::kinds::OVERSIZED_BODY => {
            // Fill entire buffer with 'A', never set EOF
            let data = vec![b'A'; buf_limit as usize];
            let len = copy_to_buf(&data, buf, buf_limit);
            len as i64
        }
        test::kinds::BODY_WITHOUT_EOF => (0i64 << 32) | 1,
        0 => (1i64 << 32) | 0,
        _ => {
            let len = copy_to_buf(b"<html><body>test</body>", buf, buf_limit);
            (1i64 << 32) | (len as i64)
        }
    }
}

pub(crate) unsafe fn write_body(_kind: i32, _body: *const u8, _len: i32) {
    // No-op in tests
}

// -------------------------------------------------------------------------
// Status Code
// -------------------------------------------------------------------------

pub(crate) unsafe fn get_status_code() -> i32 {
    200
}

pub(crate) unsafe fn set_status_code(_code: i32) {
    // No-op in tests
}

// -------------------------------------------------------------------------
// Features
// -------------------------------------------------------------------------

pub(crate) unsafe fn enable_features(_feature: i32) -> i32 {
    0 // Success
}

// -------------------------------------------------------------------------
// Source Address
// -------------------------------------------------------------------------

pub(crate) unsafe fn get_source_addr(buf: *mut u8, buf_limit: i32) -> i32 {
    copy_to_buf(b"192.168.1.1", buf, buf_limit)
}
