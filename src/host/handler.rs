use super::memory::Buffer;
use crate::host::memory;

mod http_handler;

pub fn log(level: i32, message: &[u8]) {
    unsafe { http_handler::log(level, message.as_ptr(), message.len() as i32) };
}

pub fn log_enabled(level: i32) -> bool {
    matches!(unsafe { http_handler::log_enabled(level) }, 1)
}

pub fn get_config(buffer: &Buffer) -> Option<Vec<u8>> {
    match unsafe { http_handler::get_config(buffer.data.as_ptr(), buffer.size) } {
        0 => None,
        size if size <= buffer.size => memory::to_bytes(buffer.data.as_slice(), size),
        capacity => {
            let mut buf = Vec::with_capacity(capacity as usize);
            let vec = unsafe {
                let ptr = buf.as_mut_ptr();
                let length = http_handler::get_config(ptr, capacity);
                Vec::from_raw_parts(ptr, length as usize, capacity as usize)
            };
            std::mem::forget(buf);
            Some(vec)
        }
    }
}

pub fn enable_feature(feature: i32) -> i32 {
    unsafe { http_handler::enable_features(feature) }
}

pub fn header_values(buffer: &Buffer, kind: i32, name: &[u8]) -> Vec<Vec<u8>> {
    let count_len = unsafe {
        http_handler::get_header_values(
            kind,
            name.as_ptr(),
            name.len() as i32,
            buffer.data.as_ptr(),
            buffer.size,
        )
    };
    let (count, len) = memory::split_i64(count_len);
    if len <= buffer.size {
        return memory::handle_values(buffer.data.as_slice(), count, len);
    }

    let mut buf = Vec::with_capacity(len as usize);
    let vec = unsafe {
        let ptr = buf.as_mut_ptr();
        let length =
            http_handler::get_header_values(kind, name.as_ptr(), name.len() as i32, ptr, len);
        let new_buf = Vec::from_raw_parts(ptr, length as usize, len as usize);
        memory::handle_values(new_buf.as_slice(), count, len)
    };
    std::mem::forget(buf);
    vec
}

pub fn header_names(buffer: &Buffer, kind: i32) -> Vec<Vec<u8>> {
    let count_len =
        unsafe { http_handler::get_header_names(kind, buffer.data.as_ptr(), buffer.size) };
    let (count, len) = memory::split_i64(count_len);
    if len <= buffer.size {
        return memory::handle_values(buffer.data.as_slice(), count, len);
    }
    let mut buf = Vec::with_capacity(len as usize);
    let vec = unsafe {
        let ptr = buf.as_mut_ptr();
        let length = http_handler::get_header_names(kind, ptr, len);
        let new_buf = Vec::from_raw_parts(ptr, length as usize, len as usize);
        memory::handle_values(new_buf.as_slice(), count, len)
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

pub fn source_addr(buffer: &Buffer) -> Option<Vec<u8>> {
    match unsafe { http_handler::get_source_addr(buffer.data.as_ptr(), buffer.size) } {
        size if size > 0 && size <= buffer.size => memory::to_bytes(buffer.data.as_slice(), size),
        _ => None,
    }
}

pub fn method(buffer: &Buffer) -> Option<Vec<u8>> {
    match unsafe { http_handler::get_method(buffer.data.as_ptr(), buffer.size) } {
        size if size > 0 && size <= buffer.size => memory::to_bytes(buffer.data.as_slice(), size),
        _ => None,
    }
}

pub fn set_method(method: &[u8]) {
    unsafe { http_handler::set_method(method.as_ptr(), method.len() as i32) };
}

pub fn set_uri(uri: &[u8]) {
    unsafe { http_handler::set_uri(uri.as_ptr(), uri.len() as i32) };
}

pub fn version(buffer: &Buffer) -> Option<Vec<u8>> {
    match unsafe { http_handler::get_protocol_version(buffer.data.as_ptr(), buffer.size) } {
        size if size > 0 && size <= buffer.size => memory::to_bytes(buffer.data.as_slice(), size),
        _ => None,
    }
}
pub fn uri(buffer: &Buffer) -> Option<Vec<u8>> {
    match unsafe { http_handler::get_uri(buffer.data.as_ptr(), buffer.size) } {
        size if size > 0 && size <= buffer.size => memory::to_bytes(buffer.data.as_slice(), size),
        _ => None,
    }
}

pub fn status_code() -> i32 {
    unsafe { http_handler::get_status_code() }
}

pub fn set_status_code(code: i32) {
    unsafe { http_handler::set_status_code(code) }
}

pub fn body(buffer: &Buffer, kind: i32) -> Option<Vec<u8>> {
    let mut eof = false;
    let mut size;
    let mut out = Vec::new();
    while !eof {
        (eof, size) = memory::eof_size(unsafe {
            http_handler::read_body(kind, buffer.data.as_ptr(), buffer.size)
        });
        if let Some(vec) = memory::to_bytes(buffer.data.as_slice(), size) {
            out.push(vec)
        }
    }
    if out.is_empty() {
        None
    } else {
        Some(out.concat())
    }
}

pub fn write_body(kind: i32, body: &str) {
    unsafe {
        http_handler::write_body(kind, body.as_ptr(), body.len() as i32);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_code() {
        let rc = status_code();
        assert_eq!(rc, 200);
    }
    #[test]
    fn test_config() {
        let data = "data".as_bytes().to_vec();
        let buf = Buffer::from_vec(&data);
        let rc = get_config(&buf);
        assert_eq!(rc, Some(data));
    }

    #[test]
    fn test_body() {
        let data = "data".as_bytes().to_vec();
        let buf = Buffer::from_vec(&data);
        let s = body(&buf, 1);
        assert_eq!(s, Some(data));
    }

    #[test]
    fn test_version() {
        let data = "data".as_bytes().to_vec();
        let buf = Buffer::from_vec(&data);
        let s = version(&buf);
        assert_eq!(s, Some(data));
    }
}
