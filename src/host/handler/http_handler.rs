#[rustfmt::skip]
#[link(wasm_import_module = "http_handler")]
unsafe extern "C" {
    pub(crate) unsafe fn log(level: i32, message: *const u8, message_len: i32);
    pub(crate) unsafe fn log_enabled(level: i32) -> i32;
    pub(crate) unsafe fn get_config(buf: *const u8, buf_limit: i32) -> i32;
    pub(crate) unsafe fn get_method(buf: *const u8, buf_limit: i32) -> i32;
    pub(crate) unsafe fn set_method(ptr: *const u8, message_len: i32);
    pub(crate) unsafe fn get_uri(ptr: *const u8, buf_limit: i32) -> i32;
    pub(crate) unsafe fn set_uri(ptr: *const u8, message_len: i32);
    pub(crate) unsafe fn get_protocol_version(ptr: *const u8, message_len: i32) -> i32;
    pub(crate) unsafe fn add_header_value(header_kind: i32, name_ptr: *const u8, name_len: i32, value_ptr: *const u8, value_len: i32,);
    pub(crate) unsafe fn set_header_value(header_kind: i32, name_ptr: *const u8, name_len: i32, value_ptr: *const u8, value_len: i32,);
    pub(crate) unsafe fn remove_header(header_kind: i32, name_ptr: *const u8, name_len: i32);
    pub(crate) unsafe fn get_header_names(header_kind: i32, buf: *const u8, buf_limit: i32) -> i64;
    pub(crate) unsafe fn get_header_values(header_kind: i32, name_ptr: *const u8, name_len: i32, buf: *const u8, buf_limit: i32,) -> i64;
    pub(crate) unsafe fn read_body(body_kind: i32, ptr: *const u8, buf_limit: i32) -> i64;
    pub(crate) unsafe fn write_body(body_kind: i32, ptr: *const u8, message_len: i32);
    pub(crate) unsafe fn get_status_code() -> i32;
    pub(crate) unsafe fn set_status_code(code: i32);
    pub(crate) unsafe fn enable_features(feature: i32) -> i32;
    pub(crate) unsafe fn get_source_addr(buf: *const u8, buf_limit: i32) -> i32;
}

#[cfg(test)]
pub mod overrides {

    #[unsafe(no_mangle)]
    pub extern "C" fn get_status_code() -> i32 {
        200
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn get_config(buf: *mut u8, _buf_limit: i32) -> i32 {
        let m = br#"{ "config" : "test",}"#;
        unsafe { buf.copy_from(m.as_ptr(), m.len()) };
        m.len() as i32
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn get_method(buf: *mut u8, _buf_limit: i32) -> i32 {
        let m = b"GET";
        unsafe { buf.copy_from(m.as_ptr(), m.len()) };
        m.len() as i32
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn get_protocol_version(buf: *mut u8, _message_len: i32) -> i32 {
        let m = b"HTTP/2.0";
        unsafe { buf.copy_from(m.as_ptr(), m.len()) };
        m.len() as i32
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn read_body(_body_kind: i32, buf: *mut u8, _buf_limit: i32) -> i64 {
        let m = b"<html><body>test</body>";
        unsafe { buf.copy_from(m.as_ptr(), m.len()) };
        1i64 << 32 | m.len() as i64
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn get_header_names(_header_kind: i32, buf: *mut u8, _buf_limit: i32) -> i64 {
        let m = b"X-FOO\0x-bar\0";
        unsafe { buf.copy_from(m.as_ptr(), m.len()) };
        2i64 << 32 | m.len() as i64
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn get_header_values(
        _header_kind: i32,
        _name_ptr: *const u8,
        _name_len: i32,
        buf: *mut u8,
        _buf_limit: i32,
    ) -> i64 {
        let m = b"test1\0";
        unsafe { buf.copy_from(m.as_ptr(), m.len()) };
        1i64 << 32 | m.len() as i64
    }
}
