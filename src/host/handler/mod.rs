mod ffi;
mod memory;

pub(crate) fn log(level: i32, message: &[u8]) {
    unsafe { ffi::log(level, message.as_ptr(), message.len() as i32) };
}

pub(crate) fn log_enabled(level: i32) -> bool {
    matches!(unsafe { ffi::log_enabled(level) }, 1)
}

pub(crate) fn get_config() -> Vec<u8> {
    let buffer = memory::buffer();
    match unsafe { ffi::get_config(buffer.as_ptr(), buffer.len()) } {
        size if size <= buffer.len() => buffer.as_subslice(size).to_vec(),
        capacity => {
            let mut buf = Vec::with_capacity(capacity as usize);
            let vec = unsafe {
                let ptr = buf.as_mut_ptr();
                let length = ffi::get_config(ptr, capacity);
                Vec::from_raw_parts(ptr, length as usize, capacity as usize)
            };
            std::mem::forget(buf);
            vec
        }
    }
}

pub(crate) fn enable_feature(feature: i32) -> i32 {
    unsafe { ffi::enable_features(feature) }
}

pub(crate) fn header_values(kind: i32, name: &[u8]) -> Vec<Box<[u8]>> {
    let buffer = memory::buffer();
    let count_len = unsafe {
        ffi::get_header_values(
            kind,
            name.as_ptr(),
            name.len() as i32,
            buffer.as_ptr(),
            buffer.len(),
        )
    };
    let (count, len) = split_i64(count_len);
    if len <= buffer.len() {
        return split(buffer.as_slice(), count, len);
    }

    let mut buf = Vec::with_capacity(len as usize);
    let vec = unsafe {
        let ptr = buf.as_mut_ptr();
        let length =
            ffi::get_header_values(kind, name.as_ptr(), name.len() as i32, ptr, len);
        let new_buf = Vec::from_raw_parts(ptr, length as usize, len as usize);
        split(new_buf.as_slice(), count, len)
    };
    std::mem::forget(buf);
    vec
}

pub(crate) fn header_names(kind: i32) -> Vec<Box<[u8]>> {
    let buffer = memory::buffer();
    let count_len = unsafe { ffi::get_header_names(kind, buffer.as_ptr(), buffer.len()) };
    let (count, len) = split_i64(count_len);
    if len <= buffer.len() {
        return split(buffer.as_slice(), count, len);
    }
    let mut buf = Vec::with_capacity(len as usize);
    let vec = unsafe {
        let ptr = buf.as_mut_ptr();
        let length = ffi::get_header_names(kind, ptr, len);
        let new_buf = Vec::from_raw_parts(ptr, length as usize, len as usize);
        split(new_buf.as_slice(), count, len)
    };
    std::mem::forget(buf);
    vec
}

pub(crate) fn remove_header(kind: i32, name: &[u8]) {
    unsafe { ffi::remove_header(kind, name.as_ptr(), name.len() as i32) }
}

pub(crate) fn set_header(kind: i32, name: &[u8], value: &[u8]) {
    unsafe {
        ffi::set_header_value(
            kind,
            name.as_ptr(),
            name.len() as i32,
            value.as_ptr(),
            value.len() as i32,
        )
    };
}

pub(crate) fn add_header_value(kind: i32, name: &[u8], value: &[u8]) {
    unsafe {
        ffi::add_header_value(
            kind,
            name.as_ptr(),
            name.len() as i32,
            value.as_ptr(),
            value.len() as i32,
        )
    };
}

pub(crate) fn source_addr() -> Box<[u8]> {
    let buffer = memory::buffer();
    let size = unsafe { ffi::get_source_addr(buffer.as_ptr(), buffer.len()) };
    buffer.to_boxed_slice(size)
}

pub(crate) fn method() -> Box<[u8]> {
    let buffer = memory::buffer();
    let size = unsafe { ffi::get_method(buffer.as_ptr(), buffer.len()) };
    buffer.to_boxed_slice(size)
}

pub(crate) fn set_method(method: &[u8]) {
    unsafe { ffi::set_method(method.as_ptr(), method.len() as i32) };
}

pub(crate) fn set_uri(uri: &[u8]) {
    unsafe { ffi::set_uri(uri.as_ptr(), uri.len() as i32) };
}

pub(crate) fn version() -> Box<[u8]> {
    let buffer = memory::buffer();
    let size = unsafe { ffi::get_protocol_version(buffer.as_ptr(), buffer.len()) };
    buffer.to_boxed_slice(size)
}

pub(crate) fn uri() -> Box<[u8]> {
    let buffer = memory::buffer();
    let size = unsafe { ffi::get_uri(buffer.as_ptr(), buffer.len()) };
    buffer.to_boxed_slice(size)
}

pub(crate) fn status_code() -> i32 {
    unsafe { ffi::get_status_code() }
}

pub(crate) fn set_status_code(code: i32) {
    unsafe { ffi::set_status_code(code) }
}

pub(crate) fn body(kind: i32) -> Box<[u8]> {
    let buffer = memory::buffer();
    let mut eof = false;
    let mut size;
    let mut out = Vec::new();
    while !eof {
        (eof, size) =
            eof_size(unsafe { ffi::read_body(kind, buffer.as_ptr(), buffer.len()) });
        out.push(buffer.as_subslice(size));
    }
    out.concat().into_boxed_slice()
}

pub(crate) fn write_body(kind: i32, body: &[u8]) {
    unsafe {
        ffi::write_body(kind, body.as_ptr(), body.len() as i32);
    }
}

fn split(buf: &[u8], count: i32, len: i32) -> Vec<Box<[u8]>> {
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
        assert_eq!(status_code(), 200)
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
}
