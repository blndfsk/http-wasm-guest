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
    pub(crate) unsafe fn get_config(buf: *const u8, buf_limit: i32) -> i32;
    pub(crate) unsafe fn get_method(buf: *const u8, buf_limit: i32) -> i32;
    pub(crate) unsafe fn set_method(method: *const u8, len: i32);
    pub(crate) unsafe fn get_uri(buf: *const u8, buf_limit: i32) -> i32;
    pub(crate) unsafe fn set_uri(uri: *const u8, len: i32);
    pub(crate) unsafe fn get_protocol_version(buf: *const u8, buf_limit: i32) -> i32;
    pub(crate) unsafe fn add_header_value(kind: i32, name: *const u8, name_len: i32, value: *const u8, value_len: i32);
    pub(crate) unsafe fn set_header_value(kind: i32, name: *const u8, name_len: i32, value: *const u8, value_len: i32);
    pub(crate) unsafe fn remove_header(kind: i32, name: *const u8, len: i32);
    pub(crate) unsafe fn get_header_names(kind: i32, buf: *const u8, buf_limit: i32) -> i64;
    pub(crate) unsafe fn get_header_values(kind: i32, name: *const u8, len: i32, buf: *const u8, buf_limit: i32) -> i64;
    pub(crate) unsafe fn read_body(kind: i32, buf: *const u8, buf_limit: i32) -> i64;
    pub(crate) unsafe fn write_body(kind: i32, body: *const u8, len: i32);
    pub(crate) unsafe fn get_status_code() -> i32;
    pub(crate) unsafe fn set_status_code(code: i32);
    pub(crate) unsafe fn enable_features(feature: i32) -> i32;
    pub(crate) unsafe fn get_source_addr(buf: *const u8, buf_limit: i32) -> i32;
}

// =============================================================================
// Test FFI - Mock implementations
// =============================================================================

// Re-export mock functions with the same names as the extern declarations
#[cfg(test)]
pub(crate) use mock::*;

#[cfg(test)]
pub(crate) mod mock {
    use std::cell::Cell;
    use std::ptr;

    // Thread-local flag to enable config overflow testing
    thread_local! {
        pub static CONFIG_OVERFLOW_MODE: Cell<bool> = const { Cell::new(false) };
    }

    /// Enable config overflow mode for testing
    pub fn set_config_overflow_mode(enabled: bool) {
        CONFIG_OVERFLOW_MODE.with(|f| f.set(enabled));
    }

    /// Copy bytes from source to destination buffer.
    /// Returns the number of bytes copied (limited by buf_limit).
    fn copy_to_buf(src: &[u8], buf: *const u8, buf_limit: i32) -> i32 {
        let dst = buf as *mut u8;
        let limit = buf_limit as usize;
        let len = src.len().min(limit);
        unsafe { ptr::copy_nonoverlapping(src.as_ptr(), dst, len) };
        len as i32
    }

    /// Generate large test data for overflow testing
    fn generate_large_data(base: &[u8], target_size: usize) -> Vec<u8> {
        let mut data = Vec::with_capacity(target_size);
        while data.len() < target_size {
            data.extend_from_slice(base);
        }
        data.truncate(target_size);
        data
    }

    // -------------------------------------------------------------------------
    // Logging
    // -------------------------------------------------------------------------

    pub unsafe fn log(_level: i32, _buf: *const u8, _len: i32) {
        // No-op: discard log messages in tests
    }

    pub unsafe fn log_enabled(level: i32) -> i32 {
        // Enable Error(0), Warn(1), Info(2), Debug(3); disable Trace and others
        if (0..=3).contains(&level) { 1 } else { 0 }
    }

    // -------------------------------------------------------------------------
    // Configuration
    // -------------------------------------------------------------------------

    /// Returns config data.
    /// When CONFIG_OVERFLOW_MODE is enabled, simulates buffer overflow.
    pub unsafe fn get_config(buf: *const u8, buf_limit: i32) -> i32 {
        let overflow_mode = CONFIG_OVERFLOW_MODE.with(|f| f.get());

        if overflow_mode {
            let large_size = 3000i32;
            if buf_limit < large_size {
                // First call - report that we need more space
                large_size
            } else {
                // Second call - provide the data
                let large_config = generate_large_data(br#"{ "config" : "overflow_test" }"#, large_size as usize);
                copy_to_buf(&large_config, buf, buf_limit)
            }
        } else {
            // Normal case - return small config
            let small_config = br#"{ "config" : "test1",}"#;
            copy_to_buf(small_config, buf, buf_limit)
        }
    }

    // -------------------------------------------------------------------------
    // Request Method
    // -------------------------------------------------------------------------

    pub unsafe fn get_method(buf: *const u8, buf_limit: i32) -> i32 {
        copy_to_buf(b"GET", buf, buf_limit)
    }

    pub unsafe fn set_method(_method: *const u8, _len: i32) {
        // No-op in tests
    }

    // -------------------------------------------------------------------------
    // Request URI
    // -------------------------------------------------------------------------

    pub unsafe fn get_uri(buf: *const u8, buf_limit: i32) -> i32 {
        copy_to_buf(b"https://test", buf, buf_limit)
    }

    pub unsafe fn set_uri(_uri: *const u8, _len: i32) {
        // No-op in tests
    }

    // -------------------------------------------------------------------------
    // Protocol Version
    // -------------------------------------------------------------------------

    pub unsafe fn get_protocol_version(buf: *const u8, buf_limit: i32) -> i32 {
        copy_to_buf(b"HTTP/2.0", buf, buf_limit)
    }

    // -------------------------------------------------------------------------
    // Headers
    // -------------------------------------------------------------------------

    pub unsafe fn add_header_value(_kind: i32, _name: *const u8, _name_len: i32, _value: *const u8, _value_len: i32) {
        // No-op in tests
    }

    pub unsafe fn set_header_value(_kind: i32, _name: *const u8, _name_len: i32, _value: *const u8, _value_len: i32) {
        // No-op in tests
    }

    pub unsafe fn remove_header(_kind: i32, _name: *const u8, _len: i32) {
        // No-op in tests
    }

    /// Returns header names: X-FOO, x-bar, x-baz
    /// For kind=98 (duplicate test), returns duplicate header names
    /// For kind=99 (overflow test), returns data larger than buffer
    /// Return value: count in upper 32 bits, length in lower 32 bits
    pub unsafe fn get_header_names(kind: i32, buf: *const u8, buf_limit: i32) -> i64 {
        // kind=98 triggers duplicate header name test
        if kind == 98 {
            // Return duplicate header names: X-DUP appears twice
            let data = b"X-DUP\0X-OTHER\0X-DUP\0";
            let len = copy_to_buf(data, buf, buf_limit);
            (3i64 << 32) | (len as i64)
        // kind=99 triggers overflow test
        } else if kind == 99 {
            // Generate data larger than 2048 byte buffer to trigger overflow
            let mut data = Vec::new();
            for i in 0..200 {
                data.extend_from_slice(format!("X-Header-Name-Overflow-{:03}\0", i).as_bytes());
            }
            let data_len = data.len() as i32;

            if buf_limit < data_len {
                // First call - report overflow with actual data size
                (200i64 << 32) | (data_len as i64)
            } else {
                // Second call - provide the data
                let len = copy_to_buf(&data, buf, buf_limit);
                (200i64 << 32) | (len as i64)
            }
        } else {
            let data = b"X-FOO\0x-bar\0x-baz\0";
            let len = copy_to_buf(data, buf, buf_limit);
            (3i64 << 32) | (len as i64)
        }
    }

    /// Returns header values based on name:
    /// - X-FOO: ["test1"]
    /// - x-bar: ["test2", "test3"]
    /// - x-baz: ["test4"]
    /// - X-OVERFLOW: triggers overflow test (kind=99)
    ///
    /// Return value: count in upper 32 bits, length in lower 32 bits
    pub unsafe fn get_header_values(kind: i32, name: *const u8, name_len: i32, buf: *const u8, buf_limit: i32) -> i64 {
        let name_slice = unsafe { std::slice::from_raw_parts(name, name_len as usize) };

        // kind=98 triggers duplicate header value test
        if kind == 98 {
            match name_slice {
                b"X-DUP" => (1i64 << 32) | copy_to_buf(b"dup-value\0", buf, buf_limit) as i64,
                b"X-OTHER" => (1i64 << 32) | copy_to_buf(b"other-value\0", buf, buf_limit) as i64,
                _ => 0i64,
            }
        // kind=99 with X-OVERFLOW triggers overflow test
        } else if kind == 99 && name_slice == b"X-OVERFLOW" {
            // Generate data larger than 2048 byte buffer to trigger overflow
            let mut data = Vec::new();
            for i in 0..150 {
                data.extend_from_slice(format!("overflow-header-value-{:04}\0", i).as_bytes());
            }
            let data_len = data.len() as i32;

            if buf_limit < data_len {
                // First call - report overflow with actual data size
                (150i64 << 32) | (data_len as i64)
            } else {
                // Second call - provide the data
                let len = copy_to_buf(&data, buf, buf_limit);
                (150i64 << 32) | (len as i64)
            }
        } else {
            match name_slice {
                b"X-FOO" => (1i64 << 32) | copy_to_buf(b"test1\0", buf, buf_limit) as i64,
                b"x-bar" => (2i64 << 32) | copy_to_buf(b"test2\0test3\0", buf, buf_limit) as i64,
                b"x-baz" => (1i64 << 32) | copy_to_buf(b"test4\0", buf, buf_limit) as i64,
                _ => 0i64,
            }
        }
    }

    // -------------------------------------------------------------------------
    // Body
    // -------------------------------------------------------------------------

    /// Returns mock body content with EOF flag set.
    /// Return value: EOF (1) in upper 32 bits, length in lower 32 bits
    pub unsafe fn read_body(_kind: i32, buf: *const u8, buf_limit: i32) -> i64 {
        let len = copy_to_buf(b"<html><body>test</body>", buf, buf_limit);
        (1i64 << 32) | (len as i64)
    }

    pub unsafe fn write_body(_kind: i32, _body: *const u8, _len: i32) {
        // No-op in tests
    }

    // -------------------------------------------------------------------------
    // Status Code
    // -------------------------------------------------------------------------

    pub unsafe fn get_status_code() -> i32 {
        200
    }

    pub unsafe fn set_status_code(_code: i32) {
        // No-op in tests
    }

    // -------------------------------------------------------------------------
    // Features
    // -------------------------------------------------------------------------

    pub unsafe fn enable_features(_feature: i32) -> i32 {
        0 // Success
    }

    // -------------------------------------------------------------------------
    // Source Address
    // -------------------------------------------------------------------------

    pub unsafe fn get_source_addr(buf: *const u8, buf_limit: i32) -> i32 {
        copy_to_buf(b"192.168.1.1", buf, buf_limit)
    }
}
