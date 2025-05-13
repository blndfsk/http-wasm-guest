use super::memory::{BUF, BUF_SIZE};
use crate::host::memory;
use std::str;

mod http_handler;

pub fn log(level: i32, message: &str) {
    unsafe { http_handler::log(level as i32, message.as_ptr(), message.len() as u32) };
}

pub fn log_enabled(level: i32) -> bool {
    match unsafe { http_handler::log_enabled(level) } {
        1 => return true,
        _ => return false,
    }
}

pub fn get_config() -> Option<String> {
    match unsafe { http_handler::get_config(BUF.as_ptr(), BUF_SIZE) } {
        size if size < BUF_SIZE => memory::to_string(size),
        capacity => {
            let mut buf = Vec::with_capacity(capacity as usize);
            let ptr = buf.as_mut_ptr();
            match unsafe { http_handler::get_config(ptr, capacity) } {
                length => {
                    let vec =
                        unsafe { Vec::from_raw_parts(ptr, length as usize, capacity as usize) };
                    String::from_utf8(vec).to_owned().ok()
                }
            }
        }
    }
}

pub fn enable_feature(feature: u32) -> i32 {
    return unsafe { http_handler::enable_features(feature) };
}

pub fn header_values(kind: u32, name: &str) -> Vec<Vec<u8>> {
    match unsafe {
        http_handler::get_header_values(
            kind,
            name.as_ptr(),
            name.len() as u32,
            BUF.as_ptr(),
            BUF_SIZE,
        )
    } {
        count_len if (count_len as u32) < (BUF_SIZE as u32) => memory::handle_values(count_len),
        _ => todo!(),
    }
}

pub fn header_names(kind: u32) -> Vec<Vec<u8>> {
    match unsafe { http_handler::get_header_names(kind, BUF.as_ptr(), BUF_SIZE) } {
        count_len if (count_len as u32) < (BUF_SIZE as u32) => memory::handle_values(count_len),
        _ => todo!(),
    }
}

pub fn remove_header(kind: u32, name: &str) {
    unsafe { http_handler::remove_header(kind, name.as_ptr(), name.len() as u32) }
}

pub fn set_header(kind: u32, name: &str, value: &[u8]) {
    unsafe {
        http_handler::set_header_value(
            kind as u32,
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
    match unsafe { http_handler::get_source_addr(BUF.as_ptr(), BUF_SIZE) } {
        //size if size < BUF_SIZE => str::from_utf8(&BUF[0..size as usize]).ok(),
        size if size < BUF_SIZE => memory::to_string(size),
        _ => None,
    }
}

pub fn method() -> Option<Vec<u8>> {
    match unsafe { http_handler::get_method(BUF.as_ptr(), BUF_SIZE) } {
        size if size < BUF_SIZE => memory::to_bytes(size),
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
    match unsafe { http_handler::get_protocol_version(BUF.as_ptr(), BUF_SIZE as u32) } {
        size if size < BUF_SIZE => memory::to_bytes(size),
        _ => None,
    }
}

pub fn uri() -> Option<String> {
    match unsafe { http_handler::get_uri(BUF.as_ptr(), BUF_SIZE as u32) } {
        size if size < BUF_SIZE => memory::to_string(size),
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
    let mut eof = false;
    let mut size;
    let mut out: String = String::new();
    while !eof {
        (eof, size) = memory::eof_size(unsafe {
            http_handler::read_body(kind, BUF.as_ptr(), BUF_SIZE as u32)
        });
        match memory::to_string(size) {
            Some(string) => out.push_str(&string),
            None => {}
        }
    }
    if out.is_empty() { None } else { Some(out) }
}

pub fn write_body(kind: u32, body: &str) {
    unsafe {
        http_handler::write_body(kind, body.as_ptr(), body.len() as u32);
    }
}
