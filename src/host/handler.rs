use super::memory::{BUFFER, Buffer};
use crate::host::memory;
use std::str;

mod http_handler;

pub fn log(level: i32, message: &str) {
    unsafe { http_handler::log(level, message.as_ptr(), message.len() as u32) };
}

pub fn log_enabled(level: i32) -> bool {
    matches!(unsafe { http_handler::log_enabled(level) }, 1)
}

pub fn get_config() -> Option<String> {
    __get_config(&BUFFER)
}
fn __get_config(buffer: &Buffer) -> Option<String> {
    match unsafe { http_handler::get_config(buffer.data.as_ptr(), buffer.size) } {
        size if size <= buffer.size => memory::to_string(buffer.data.as_slice(), size),
        capacity => {
            let mut buf = Vec::with_capacity(capacity as usize);
            let ptr = buf.as_mut_ptr();
            let length = unsafe { http_handler::get_config(ptr, capacity) };
            let vec = unsafe { Vec::from_raw_parts(ptr, length as usize, capacity as usize) };
            std::mem::forget(buf);
            String::from_utf8(vec).to_owned().ok()
        }
    }
}

pub fn enable_feature(feature: u32) -> i32 {
    unsafe { http_handler::enable_features(feature) }
}

pub fn header_values(kind: u32, name: &str) -> Vec<Vec<u8>> {
    __header_values(&BUFFER, kind, name)
}
fn __header_values(buffer: &Buffer, kind: u32, name: &str) -> Vec<Vec<u8>> {
    match unsafe {
        http_handler::get_header_values(
            kind,
            name.as_ptr(),
            name.len() as u32,
            buffer.data.as_ptr(),
            buffer.size,
        )
    } {
        count_len if (count_len as u32) <= (buffer.size as u32) => {
            memory::handle_values(buffer.data.as_slice(), count_len)
        }
        _ => todo!(),
    }
}
pub fn header_names(kind: u32) -> Vec<Vec<u8>> {
    __header_names(&BUFFER, kind)
}
fn __header_names(buffer: &Buffer, kind: u32) -> Vec<Vec<u8>> {
    match unsafe { http_handler::get_header_names(kind, buffer.data.as_ptr(), buffer.size) } {
        count_len if (count_len as u32) <= (buffer.size as u32) => {
            memory::handle_values(buffer.data.as_slice(), count_len)
        }
        _ => todo!(),
    }
}

pub fn remove_header(kind: u32, name: &str) {
    unsafe { http_handler::remove_header(kind, name.as_ptr(), name.len() as u32) }
}

pub fn set_header(kind: u32, name: &str, value: &[u8]) {
    unsafe {
        http_handler::set_header_value(
            kind,
            name.as_ptr(),
            name.len() as u32,
            value.as_ptr(),
            value.len() as u32,
        )
    };
}

pub fn add_header_value(kind: u32, name: &str, value: &[u8]) {
    unsafe {
        http_handler::add_header_value(
            kind,
            name.as_ptr(),
            name.len() as u32,
            value.as_ptr(),
            value.len() as u32,
        )
    };
}

pub fn source_addr() -> Option<String> {
    __source_addr(&BUFFER)
}
fn __source_addr(buffer: &Buffer) -> Option<String> {
    match unsafe { http_handler::get_source_addr(buffer.data.as_ptr(), buffer.size) } {
        size if size <= buffer.size => memory::to_string(buffer.data.as_slice(), size),
        _ => None,
    }
}

pub fn method() -> Option<Vec<u8>> {
    __method(&BUFFER)
}
fn __method(buffer: &Buffer) -> Option<Vec<u8>> {
    match unsafe { http_handler::get_method(buffer.data.as_ptr(), buffer.size) } {
        size if size <= buffer.size => memory::to_bytes(buffer.data.as_slice(), size),
        _ => None,
    }
}

pub fn set_method(method: &str) {
    unsafe { http_handler::set_method(method.as_ptr(), method.len() as u32) };
}

pub fn set_uri(uri: &str) {
    unsafe { http_handler::set_uri(uri.as_ptr(), uri.len() as u32) };
}

pub fn version() -> Option<Vec<u8>> {
    __version(&BUFFER)
}
fn __version(buffer: &Buffer) -> Option<Vec<u8>> {
    match unsafe { http_handler::get_protocol_version(buffer.data.as_ptr(), buffer.size as u32) } {
        size if size <= buffer.size => memory::to_bytes(buffer.data.as_slice(), size),
        _ => None,
    }
}
pub fn uri() -> Option<String> {
    __uri(&BUFFER)
}
fn __uri(buffer: &Buffer) -> Option<String> {
    match unsafe { http_handler::get_uri(buffer.data.as_ptr(), buffer.size as u32) } {
        size if size <= buffer.size => memory::to_string(buffer.data.as_slice(), size),
        _ => None,
    }
}

pub fn status_code() -> i32 {
    unsafe { http_handler::get_status_code() }
}

pub fn set_status_code(code: i32) {
    unsafe { http_handler::set_status_code(code) }
}

pub fn body(kind: u32) -> Option<String> {
    __body(&BUFFER, kind)
}
fn __body(buffer: &Buffer, kind: u32) -> Option<String> {
    let mut eof = false;
    let mut size;
    let mut out: String = String::new();
    while !eof {
        (eof, size) = memory::eof_size(unsafe {
            http_handler::read_body(kind, buffer.data.as_ptr(), buffer.size as u32)
        });
        if let Some(string) = memory::to_string(buffer.data.as_slice(), size) {
            out.push_str(&string)
        }
    }
    if out.is_empty() { None } else { Some(out) }
}

pub fn write_body(kind: u32, body: &str) {
    unsafe {
        http_handler::write_body(kind, body.as_ptr(), body.len() as u32);
    }
}

#[cfg(test)]
mod tests {
    use crate::Type;

    use super::*;

    #[test]
    fn test_status_code() {
        let rc = status_code();
        assert_eq!(rc, 200);
    }
    #[test]
    fn test_config() {
        let data = "data";
        let buf = Buffer::from(data.as_bytes(), data.len());
        let rc = __get_config(&buf);
        assert_eq!(rc, Some(data.to_string()));
    }

    #[test]
    fn test_body() {
        let data = "data";
        let buf = Buffer::from(data.as_bytes(), data.len());
        let s = __body(&buf, Type::Request as u32);
        assert_eq!(s, Some(data.to_string()));
    }

    #[test]
    fn test_version() {
        let data = "data";
        let buf = Buffer::from(data.as_bytes(), data.len());
        let s = __version(&buf);
        assert_eq!(s, Some(data.as_bytes().to_vec()));
    }
}
