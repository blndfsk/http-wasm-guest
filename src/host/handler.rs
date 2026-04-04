use crate::memory;
mod ffi;

const MAX_BODY_SIZE: usize = 16 * 1024 * 1024; // 16 MB

pub(crate) fn log(level: i32, message: &[u8]) {
    unsafe { ffi::log(level, message.as_ptr(), as_i32(message.len())) };
}

pub(crate) fn log_enabled(level: i32) -> bool {
    (unsafe { ffi::log_enabled(level) }) != 0
}

pub(crate) fn get_config() -> Box<[u8]> {
    read_buf(|buf, limit| unsafe { ffi::get_config(buf, limit) })
}

pub(crate) fn enable_feature(feature: i32) -> i32 {
    unsafe { ffi::enable_features(feature) }
}

pub(crate) fn header_values(kind: i32, name: &[u8]) -> Vec<Box<[u8]>> {
    read_buf_multi(|buf, limit| unsafe { ffi::get_header_values(kind, name.as_ptr(), as_i32(name.len()), buf, limit) })
}

pub(crate) fn header_names(kind: i32) -> Vec<Box<[u8]>> {
    read_buf_multi(|buf, limit| unsafe { ffi::get_header_names(kind, buf, limit) })
}

pub(crate) fn remove_header(kind: i32, name: &[u8]) {
    unsafe { ffi::remove_header(kind, name.as_ptr(), as_i32(name.len())) }
}

pub(crate) fn set_header(kind: i32, name: &[u8], value: &[u8]) {
    unsafe { ffi::set_header_value(kind, name.as_ptr(), as_i32(name.len()), value.as_ptr(), as_i32(value.len())) };
}

pub(crate) fn add_header_value(kind: i32, name: &[u8], value: &[u8]) {
    unsafe { ffi::add_header_value(kind, name.as_ptr(), as_i32(name.len()), value.as_ptr(), as_i32(value.len())) };
}

pub(crate) fn source_addr() -> Box<[u8]> {
    read_buf(|buf, limit| unsafe { ffi::get_source_addr(buf, limit) })
}

pub(crate) fn method() -> Box<[u8]> {
    read_buf(|buf, limit| unsafe { ffi::get_method(buf, limit) })
}

pub(crate) fn set_method(method: &[u8]) {
    unsafe { ffi::set_method(method.as_ptr(), as_i32(method.len())) };
}

pub(crate) fn set_uri(uri: &[u8]) {
    unsafe { ffi::set_uri(uri.as_ptr(), as_i32(uri.len())) };
}

pub(crate) fn version() -> Box<[u8]> {
    read_buf(|buf, limit| unsafe { ffi::get_protocol_version(buf, limit) })
}

pub(crate) fn uri() -> Box<[u8]> {
    read_buf(|buf, limit| unsafe { ffi::get_uri(buf, limit) })
}

pub(crate) fn status_code() -> i32 {
    unsafe { ffi::get_status_code() }
}

pub(crate) fn set_status_code(code: i32) {
    unsafe { ffi::set_status_code(code) }
}

pub(crate) fn body(kind: i32) -> Box<[u8]> {
    let mut out = Vec::new();
    let max_iterations = MAX_BODY_SIZE / memory::with_buffer(|b| b.capacity());
    for _ in 0..max_iterations {
        let eof = memory::with_buffer(|buffer| {
            let (eof, size) = eof_size(unsafe { ffi::read_body(kind, buffer.as_mut_ptr(), as_i32(buffer.capacity())) });
            out.extend_from_slice(buffer.as_subslice(size.max(0) as usize));
            eof
        });
        if eof || out.len() >= MAX_BODY_SIZE {
            break;
        }
    }
    out.into_boxed_slice()
}

pub(crate) fn write_body(kind: i32, body: &[u8]) {
    unsafe {
        ffi::write_body(kind, body.as_ptr(), as_i32(body.len()));
    }
}

/// Converts a `usize` to `i32` for the FFI boundary.
/// Panics instead of wrapping to negative values.
fn as_i32(n: usize) -> i32 {
    i32::try_from(n).expect("length exceeds i32::MAX")
}

/// Calls an FFI function that writes into a buffer and returns the actual size.
/// If the data exceeds the shared buffer, a larger allocation is made and the call is retried.
fn read_buf(f: impl Fn(*mut u8, i32) -> i32) -> Box<[u8]> {
    memory::with_buffer(|buffer| {
        let size = f(buffer.as_mut_ptr(), as_i32(buffer.capacity())).max(0) as usize;
        if size <= buffer.capacity() {
            return buffer.to_boxed_slice(size);
        }
        let mut buf = vec![0u8; size];
        let length = f(buf.as_mut_ptr(), as_i32(size)).max(0) as usize;
        buf.truncate(length);
        buf.into_boxed_slice()
    })
}

/// Like `read_buf`, but for FFI functions that return a packed i64 (count << 32 | len)
/// and NUL-delimited multi-value data. Handles the overflow-retry pattern and splits
/// the result into individual byte slices.
fn read_buf_multi(f: impl Fn(*mut u8, i32) -> i64) -> Vec<Box<[u8]>> {
    memory::with_buffer(|buffer| {
        let (count, len) = split_i64(f(buffer.as_mut_ptr(), as_i32(buffer.capacity())));
        let len = len.max(0);
        if len as usize <= buffer.capacity() {
            return split(buffer.as_slice(), count, len);
        }

        let mut buf = vec![0u8; len as usize];
        let (count, len) = split_i64(f(buf.as_mut_ptr(), len));
        split(&buf, count, len)
    })
}

fn split(buf: &[u8], count: i32, len: i32) -> Vec<Box<[u8]>> {
    let len = len.max(0) as usize;
    let out: Vec<Box<[u8]>> = buf[..len].split(|&b| b == 0).filter(|s| !s.is_empty()).map(Box::from).collect();
    debug_assert_eq!(out.len(), count as usize, "split count mismatch: host reported {count} items but found {}", out.len());
    out
}

fn split_i64(n: i64) -> (i32, i32) {
    (
        (n >> 32) as i32, //upper count
        n as i32,         //lower len
    )
}

fn eof_size(n: i64) -> (bool, i32) {
    let (v, size) = split_i64(n);
    (v == 1, size)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_status_code() {
        assert_eq!(status_code(), 200);
    }

    #[test]
    fn test_split_i64() {
        let (a, b) = split_i64(2 << 32 | 28);
        assert_eq!((a, b), (2, 28));
    }

    #[test]
    fn test_method() {
        let m = method();
        assert_eq!(b"GET", m.as_ref());
    }

    #[test]
    fn test_eof_size_with_eof() {
        // EOF flag set (1 in upper 32 bits), size 100 in lower 32 bits
        let (eof, size) = eof_size(1i64 << 32 | 100);
        assert!(eof);
        assert_eq!(size, 100);
    }

    #[test]
    fn test_eof_size_without_eof() {
        // EOF flag not set (0 in upper 32 bits), size 50 in lower 32 bits
        #[allow(clippy::identity_op)]
        let (eof, size) = eof_size(0i64 << 32 | 50);
        assert!(!eof);
        assert_eq!(size, 50);
    }

    #[test]
    fn test_split_nul_single() {
        let data = b"hello\0";
        let result = split(data, 1, data.len() as i32);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].as_ref(), b"hello");
    }

    #[test]
    fn test_split_nul_multiple() {
        let data = b"foo\0bar\0baz\0";
        let result = split(data, 3, data.len() as i32);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].as_ref(), b"foo");
        assert_eq!(result[1].as_ref(), b"bar");
        assert_eq!(result[2].as_ref(), b"baz");
    }

    #[test]
    fn test_split_nul_empty() {
        let data = b"";
        let result = split(data, 0, data.len() as i32);
        assert!(result.is_empty());
    }

    #[test]
    fn test_body_read() {
        // Test reading body - mock returns HTML content with EOF
        let content = body(0);
        assert!(!content.is_empty());
        assert!(content.starts_with(b"<html>"));
    }

    #[test]
    fn test_write_body() {
        // Should not panic - mock accepts any body
        write_body(0, b"test body content");
    }

    #[test]
    fn test_version() {
        let v = version();
        assert_eq!(v.as_ref(), b"HTTP/2.0");
    }

    #[test]
    fn test_uri() {
        let u = uri();
        assert_eq!(u.as_ref(), b"https://test");
    }

    #[test]
    fn test_source_addr() {
        let addr = source_addr();
        assert_eq!(addr.as_ref(), b"192.168.1.1");
    }

    #[test]
    fn test_set_method() {
        // Should not panic - mock accepts any method
        set_method(b"POST");
    }

    #[test]
    fn test_set_uri() {
        // Should not panic - mock accepts any URI
        set_uri(b"/new/path");
    }

    #[test]
    fn test_set_status_code() {
        // Should not panic - mock accepts any status code
        set_status_code(404);
    }

    #[test]
    fn test_get_config() {
        let config = get_config();
        let config_str = std::str::from_utf8(&config).unwrap();
        assert!(config_str.contains("config"));
    }

    #[test]
    fn test_enable_feature() {
        // Should return 0 (success) from mock
        let result = enable_feature(1);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_log() {
        // Should not panic - mock accepts log calls
        log(2, b"test message");
    }

    #[test]
    fn test_log_enabled() {
        // Mock enables levels 0-3
        assert!(log_enabled(0));
        assert!(log_enabled(2));
        assert!(!log_enabled(-1));
    }

    #[test]
    fn test_header_names() {
        let names = header_names(0);
        assert_eq!(names.len(), 3);
    }

    #[test]
    fn test_header_values_existing() {
        let values = header_values(0, b"X-FOO");
        assert_eq!(values.len(), 1);
        assert_eq!(values[0].as_ref(), b"test1");
    }

    #[test]
    fn test_header_values_multiple() {
        let values = header_values(0, b"x-bar");
        assert_eq!(values.len(), 2);
    }

    #[test]
    fn test_header_values_nonexistent() {
        let values = header_values(0, b"X-UNKNOWN");
        assert!(values.is_empty());
    }

    #[test]
    fn test_remove_header() {
        // Should not panic - mock accepts header removal
        remove_header(0, b"X-FOO");
    }

    #[test]
    fn test_set_header() {
        // Should not panic - mock accepts header set
        set_header(0, b"X-New", b"value");
    }

    #[test]
    fn test_add_header_value() {
        // Should not panic - mock accepts header add
        add_header_value(0, b"X-Existing", b"new-value");
    }

    // =========================================================================
    // Buffer Overflow Tests
    // =========================================================================

    #[test]
    fn test_header_names_overflow() {
        // kind=99 triggers overflow simulation in mock
        // First call returns size > buffer (2048), second call provides data
        let names = header_names(99);
        // Should have 100 headers from the overflow mock
        assert_eq!(names.len(), 100);
        // Verify first header name format
        assert!(names[0].starts_with(b"X-Header-Name-Overflow-"));
    }

    #[test]
    fn test_header_values_overflow() {
        // kind=99 with X-OVERFLOW triggers overflow simulation
        // Data exceeds 2048 byte buffer
        let values = header_values(99, b"X-OVERFLOW");
        // Should have 100 values from the overflow mock
        assert_eq!(values.len(), 100);
        // Verify value format
        assert!(values[0].starts_with(b"overflow-header-value-"));
    }

    #[test]
    fn test_get_config_overflow() {
        // Enable overflow mode for this test
        ffi::mock::set_config_overflow_mode(true);

        // This should trigger the overflow branch in get_config
        let config = get_config();

        // Verify we got the large config data
        assert!(config.len() > 2048);
        let config_str = std::str::from_utf8(&config).unwrap();
        assert!(config_str.contains("overflow_test"));

        // Reset overflow mode
        ffi::mock::set_config_overflow_mode(false);
    }

    #[test]
    #[should_panic(expected = "split count mismatch")]
    fn test_split_count_mismatch() {
        // count=5 but data only contains 3 items → debug_assert fires
        let data = b"one\0two\0three\0";
        split(data, 5, data.len() as i32);
    }
}
