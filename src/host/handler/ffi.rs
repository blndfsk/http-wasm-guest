#[rustfmt::skip]
#[link(wasm_import_module = "http_handler")]
unsafe extern "C" {
    pub(crate) unsafe fn log(level: i32, buf: *const u8, length: i32);
    pub(crate) unsafe fn log_enabled(level: i32) -> i32;
    pub(crate) unsafe fn get_config(buf: *const u8, limit: i32) -> i32;
    pub(crate) unsafe fn get_method(buf: *const u8, limit: i32) -> i32;
    pub(crate) unsafe fn set_method(ptr: *const u8, length: i32);
    pub(crate) unsafe fn get_uri(ptr: *const u8, buf_limit: i32) -> i32;
    pub(crate) unsafe fn set_uri(ptr: *const u8, message_len: i32);
    pub(crate) unsafe fn get_protocol_version(ptr: *const u8, message_len: i32) -> i32;
    pub(crate) unsafe fn add_header_value(kind: i32, name_ptr: *const u8, name_len: i32, value_ptr: *const u8, value_len: i32,);
    pub(crate) unsafe fn set_header_value(kind: i32, name_ptr: *const u8, name_len: i32, value_ptr: *const u8, value_len: i32,);
    pub(crate) unsafe fn remove_header(kind: i32, name_ptr: *const u8, name_len: i32);
    pub(crate) unsafe fn get_header_names(kind: i32, buf: *const u8, buf_limit: i32) -> i64;
    pub(crate) unsafe fn get_header_values(kind: i32, name_ptr: *const u8, name_len: i32, buf: *const u8, buf_limit: i32,) -> i64;
    pub(crate) unsafe fn read_body(kind: i32, ptr: *const u8, buf_limit: i32) -> i64;
    pub(crate) unsafe fn write_body(kind: i32, ptr: *const u8, message_len: i32);
    pub(crate) unsafe fn get_status_code() -> i32;
    pub(crate) unsafe fn set_status_code(code: i32);
    pub(crate) unsafe fn enable_features(feature: i32) -> i32;
    pub(crate) unsafe fn get_source_addr(buf: *const u8, buf_limit: i32) -> i32;
}

#[cfg(test)]
pub mod overrides {
    use std::ptr;

    #[unsafe(no_mangle)]
    pub extern "C" fn log_enabled(_level: i32) -> i32 {
        0
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn log(_level: i32, _message: *const u8, _message_len: i32) {}

    #[unsafe(no_mangle)]
    pub extern "C" fn get_status_code() -> i32 {
        200
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn set_status_code(_code: i32) {}

    #[unsafe(no_mangle)]
    pub extern "C" fn get_uri(buf: *const u8, buf_limit: i32) -> i32 {
        copy(br#"https://test"#, buf, buf_limit)
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn set_uri(_buf: *const u8, _message_len: i32) {}

    #[unsafe(no_mangle)]
    pub extern "C" fn get_config(buf: *mut u8, buf_limit: i32) -> i32 {
        copy(br#"{ "config" : "test1",}"#, buf, buf_limit)
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn get_method(buf: *const u8, buf_limit: i32) -> i32 {
        copy(b"GET", buf, buf_limit)
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn set_method(_ptr: *const u8, _message_len: i32) {}

    #[unsafe(no_mangle)]
    pub extern "C" fn get_protocol_version(buf: *const u8, message_len: i32) -> i32 {
        copy(b"HTTP/2.0", buf, message_len)
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn add_header_value(_kind: i32, _name_ptr: *const u8, _name_len: i32, _value_ptr: *const u8, _value_len: i32) {
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn set_header_value(_kind: i32, _name_ptr: *const u8, _name_len: i32, _value_ptr: *const u8, _value_len: i32) {
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn remove_header(_kind: i32, _name_ptr: *const u8, _name_len: i32) {}

    #[unsafe(no_mangle)]
    pub extern "C" fn read_body(_kind: i32, buf: *mut u8, buf_limit: i32) -> i64 {
        1i64 << 32 | copy(b"<html><body>test</body>", buf, buf_limit) as i64
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn write_body(_kind: i32, _ptr: *const u8, _message_len: i32) {}

    #[unsafe(no_mangle)]
    pub extern "C" fn get_header_names(_kind: i32, buf: *mut u8, buf_limit: i32) -> i64 {
        3i64 << 32 | copy(b"X-FOO\0x-bar\0x-baz\0", buf, buf_limit) as i64
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn get_header_values(_kind: i32, name_ptr: *const u8, name_len: i32, buf: *mut u8, buf_limit: i32) -> i64 {
        let name = unsafe { std::slice::from_raw_parts(name_ptr, name_len as usize) };
        match name {
            b"X-FOO" => 1i64 << 32 | copy(b"test1\0", buf, buf_limit) as i64,
            b"x-bar" => 2i64 << 32 | copy(b"test2\0test3\0", buf, buf_limit) as i64,
            b"x-baz" => 1i64 << 32 | copy(b"test4\0", buf, buf_limit) as i64,
            _ => 0i64,
        }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn get_source_addr(buf: *mut u8, buf_limit: i32) -> i32 {
        copy(b"192.168.1.1", buf, buf_limit)
    }

    fn copy(src: &[u8], dst: *const u8, limit: i32) -> i32 {
        let mut_dst = dst as *mut u8;
        let len = limit as usize;
        let len = if src.len() > len { len } else { src.len() };
        unsafe { ptr::copy(src.as_ptr(), mut_dst, len) };
        len as i32
    }
}
