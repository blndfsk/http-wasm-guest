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

    // =========================================================================
    // Internal Helper Tests
    // =========================================================================

    #[test]
    fn test_split_i64() {
        let (a, b) = split_i64(2 << 32 | 28);
        assert_eq!((a, b), (2, 28));
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
    fn test_split_nul_empty_elem() {
        let data = b"\0test1\0\0test2\0";
        let result = split(data, 2, data.len() as i32);
        assert_eq!(result.len(), 2);
    }

    // =========================================================================
    // Buffer Overflow / Retry Tests
    // =========================================================================

    #[test]
    fn test_read_buf_overflow() {
        let large_data: Vec<u8> = (0..3000).map(|i| b'A' + (i % 26) as u8).collect();
        let large_data_clone = large_data.clone();
        let result = read_buf(|buf, limit| {
            let needed = large_data_clone.len() as i32;
            if limit < needed {
                // First call: report that we need more space
                needed
            } else {
                // Second call: write the data
                unsafe { std::ptr::copy_nonoverlapping(large_data_clone.as_ptr(), buf, large_data_clone.len()) };
                needed
            }
        });
        assert_eq!(result.len(), 3000);
        assert_eq!(result.as_ref(), large_data.as_slice());
    }

    #[test]
    fn test_read_buf_multi_overflow() {
        // Build NUL-delimited data larger than 2048-byte shared buffer
        let mut data = Vec::new();
        let count = 120;
        for i in 0..count {
            data.extend_from_slice(format!("overflow-item-{:04}\0", i).as_bytes());
        }
        let data_clone = data.clone();
        let result = read_buf_multi(|buf, limit| {
            let needed = data_clone.len() as i32;
            if limit < needed {
                (count as i64) << 32 | (needed as i64)
            } else {
                unsafe { std::ptr::copy_nonoverlapping(data_clone.as_ptr(), buf, data_clone.len()) };
                (count as i64) << 32 | (needed as i64)
            }
        });
        assert_eq!(result.len(), count);
        assert_eq!(result[0].as_ref(), b"overflow-item-0000");
        assert_eq!(result[119].as_ref(), b"overflow-item-0119");
    }

    #[test]
    #[should_panic(expected = "split count mismatch")]
    fn test_split_count_mismatch() {
        // count=5 but data only contains 3 items → debug_assert fires
        let data = b"one\0two\0three\0";
        split(data, 5, data.len() as i32);
    }

    #[test]
    fn test_body_max_size_limit() {
        // kind=99 returns full buffer chunks without EOF
        let content = body(99);
        assert!(content.len() >= MAX_BODY_SIZE);
        assert!(content.iter().all(|&b| b == b'A'));
    }
}
