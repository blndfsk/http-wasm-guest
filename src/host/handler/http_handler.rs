#[link(wasm_import_module = "http_handler")]
unsafe extern "C" {
    pub(crate) unsafe fn log(level: i32, message: *const u8, message_len: u32);
    /// get_config writes configuration from the host to memory if it exists and
    /// isn't larger than the `buf_limit`. The result is its length in bytes.
    pub(crate) unsafe fn get_config(buf: *const u8, buf_limit: i32) -> i32;
    pub(crate) unsafe fn get_method(buf: *const u8, buf_limit: i32) -> i32;
    pub(crate) unsafe fn set_method(ptr: *const u8, message_len: u32);
    pub(crate) unsafe fn get_uri(ptr: *const u8, buf_limit: u32) -> i32;
    pub(crate) unsafe fn set_uri(ptr: *const u8, message_len: u32);
    pub(crate) unsafe fn get_protocol_version(ptr: *const u8, message_len: u32) -> i32;
    pub(crate) unsafe fn add_header_value(
        header_kind: u32,
        name_ptr: *const u8,
        name_len: u32,
        value_ptr: *const u8,
        value_len: u32,
    );
    pub(crate) unsafe fn set_header_value(
        header_kind: u32,
        name_ptr: *const u8,
        name_len: u32,
        value_ptr: *const u8,
        value_len: u32,
    );
    pub(crate) unsafe fn remove_header(header_kind: u32, name_ptr: *const u8, name_len: u32);
    pub(crate) unsafe fn get_header_names(header_kind: u32, buf: *const u8, buf_limit: i32) -> i64;
    pub(crate) unsafe fn get_header_values(
        header_kind: u32,
        name_ptr: *const u8,
        name_len: u32,
        buf: *const u8,
        buf_limit: i32,
    ) -> i64;
    pub(crate) unsafe fn log_enabled(level: i32) -> i32;
    pub(crate) unsafe fn read_body(body_kind: u32, ptr: *const u8, buf_limit: u32) -> i64;
    pub(crate) unsafe fn write_body(body_kind: u32, ptr: *const u8, message_len: u32);
    pub(crate) unsafe fn get_status_code() -> i32;
    pub(crate) unsafe fn set_status_code(code: i32);
    pub(crate) unsafe fn enable_features(feature: u32) -> i32;
    pub(crate) unsafe fn get_source_addr(buf: *const u8, buf_limit: i32) -> i32;
}
