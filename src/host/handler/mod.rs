use crate::host::handler::memory::Buffer;

mod http_handler;
mod memory;

pub fn log(level: i32, message: &[u8]) {
    unsafe { http_handler::log(level, message.as_ptr(), message.len() as i32) };
}

pub fn log_enabled(level: i32) -> bool {
    matches!(unsafe { http_handler::log_enabled(level) }, 1)
}

pub fn get_config() -> Vec<u8> {
    let buffer = memory::buffer();
    match unsafe { http_handler::get_config(buffer.data.as_ptr(), buffer.len()) } {
        size if size <= buffer.len() => buffer.data.as_slice()[..size as usize].to_vec(),
        capacity => {
            let mut buf = Vec::with_capacity(capacity as usize);
            let vec = unsafe {
                let ptr = buf.as_mut_ptr();
                let length = http_handler::get_config(ptr, capacity);
                Vec::from_raw_parts(ptr, length as usize, capacity as usize)
            };
            std::mem::forget(buf);
            vec
        }
    }
}

pub fn enable_feature(feature: i32) -> i32 {
    unsafe { http_handler::enable_features(feature) }
}

pub fn header_values(kind: i32, name: &[u8]) -> Vec<Box<[u8]>> {
    let buffer = memory::buffer();
    let count_len = unsafe {
        http_handler::get_header_values(
            kind,
            name.as_ptr(),
            name.len() as i32,
            buffer.data.as_ptr(),
            buffer.len(),
        )
    };
    let (count, len) = split_i64(count_len);
    if len <= buffer.len() {
        return handle_values(buffer.data.as_slice(), count, len);
    }

    let mut buf = Vec::with_capacity(len as usize);
    let vec = unsafe {
        let ptr = buf.as_mut_ptr();
        let length =
            http_handler::get_header_values(kind, name.as_ptr(), name.len() as i32, ptr, len);
        let new_buf = Vec::from_raw_parts(ptr, length as usize, len as usize);
        handle_values(new_buf.as_slice(), count, len)
    };
    std::mem::forget(buf);
    vec
}

pub fn header_names(kind: i32) -> Vec<Box<[u8]>> {
    let buffer = memory::buffer();
    let count_len =
        unsafe { http_handler::get_header_names(kind, buffer.data.as_ptr(), buffer.len()) };
    let (count, len) = split_i64(count_len);
    if len <= buffer.len() {
        return handle_values(buffer.data.as_slice(), count, len);
    }
    let mut buf = Vec::with_capacity(len as usize);
    let vec = unsafe {
        let ptr = buf.as_mut_ptr();
        let length = http_handler::get_header_names(kind, ptr, len);
        let new_buf = Vec::from_raw_parts(ptr, length as usize, len as usize);
        handle_values(new_buf.as_slice(), count, len)
    };
    std::mem::forget(buf);
    vec
}

pub fn remove_header(kind: i32, name: &[u8]) {
    unsafe { http_handler::remove_header(kind, name.as_ptr(), name.len() as i32) }
}

pub fn set_header(kind: i32, name: &[u8], value: &[u8]) {
    unsafe {
        http_handler::set_header_value(
            kind,
            name.as_ptr(),
            name.len() as i32,
            value.as_ptr(),
            value.len() as i32,
        )
    };
}

pub fn add_header_value(kind: i32, name: &[u8], value: &[u8]) {
    unsafe {
        http_handler::add_header_value(
            kind,
            name.as_ptr(),
            name.len() as i32,
            value.as_ptr(),
            value.len() as i32,
        )
    };
}

pub fn source_addr() -> Box<[u8]> {
    let buffer = memory::buffer();
    let size = unsafe { http_handler::get_source_addr(buffer.data.as_ptr(), buffer.len()) };
    extract_bytes(buffer, size)
}

pub fn method() -> Box<[u8]> {
    let buffer = memory::buffer();
    let size = unsafe { http_handler::get_method(buffer.data.as_ptr(), buffer.len()) };
    extract_bytes(buffer, size)
}

pub fn set_method(method: &[u8]) {
    unsafe { http_handler::set_method(method.as_ptr(), method.len() as i32) };
}

pub fn set_uri(uri: &[u8]) {
    unsafe { http_handler::set_uri(uri.as_ptr(), uri.len() as i32) };
}

pub fn version() -> Box<[u8]> {
    let buffer = memory::buffer();
    let size = unsafe { http_handler::get_protocol_version(buffer.data.as_ptr(), buffer.len()) };
    extract_bytes(buffer, size)
}
pub fn uri() -> Box<[u8]> {
    let buffer = memory::buffer();
    let size = unsafe { http_handler::get_uri(buffer.data.as_ptr(), buffer.len()) };
    extract_bytes(buffer, size)
}

pub fn status_code() -> i32 {
    unsafe { http_handler::get_status_code() }
}

pub fn set_status_code(code: i32) {
    unsafe { http_handler::set_status_code(code) }
}

pub fn body(kind: i32) -> Box<[u8]> {
    let buffer = memory::buffer();
    let mut eof = false;
    let mut size;
    let mut out = Vec::new();
    while !eof {
        (eof, size) =
            eof_size(unsafe { http_handler::read_body(kind, buffer.data.as_ptr(), buffer.len()) });
        out.push(&buffer.data[0..size as usize])
    }
    out.concat().into_boxed_slice()
}

pub fn write_body(kind: i32, body: &[u8]) {
    unsafe {
        http_handler::write_body(kind, body.as_ptr(), body.len() as i32);
    }
}

fn extract_bytes(buffer: &Buffer, size: i32) -> Box<[u8]> {
    buffer.data[0..size as usize].to_vec().into_boxed_slice()
}

fn handle_values(buf: &[u8], count: i32, len: i32) -> Vec<Box<[u8]>> {
    let src = &buf[0..len as usize];
    let mut out = Vec::with_capacity(count as usize);
    for bytes in split_u8_nul(src) {
        out.push(bytes.to_vec().into_boxed_slice());
    }
    out
}

fn split_u8_nul(src: &[u8]) -> Vec<&[u8]> {
    let mut out = Vec::new();
    let mut start_index: usize = 0;
    for (i, n) in src.iter().enumerate() {
        if *n == b'\0' {
            let t = &src[start_index..i];
            out.push(t);
            start_index = i + 1; // skip NUL byte
        }
    }
    out
}
pub fn split_i64(n: i64) -> (i32, i32) {
    (
        (n >> 32) as i32, //upper count
        n as i32,         //lower len
    )
}

pub fn eof_size(n: i64) -> (bool, i32) {
    let (v, size) = split_i64(n);
    (v == 1, size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_i64() {
        let (a, b) = split_i64(2 << 32 | 28);
        assert_eq!((a, b), (2, 28));
    }

    #[test]
    fn test_extract() {
        let c = b"test";
        let buf = &memory::Buffer::from_vec(c);
        let r = extract_bytes(buf, c.len() as i32);
        assert_eq!(b"test", r.as_ref());
    }
    #[test]
    fn test_extract_empty() {
        let c = b"";
        let buf = &memory::Buffer::from_vec(c);
        let r = extract_bytes(buf, c.len() as i32);
        assert!(r.is_empty());
    }

    #[test]
    fn test_method() {
        let m = method();
        assert!(!m.is_empty());
        assert_eq!(b"GET", m.as_ref());
    }
}
