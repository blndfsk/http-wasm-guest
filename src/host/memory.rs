use std::str;
use std::sync::LazyLock;

pub const BUF_SIZE: i32 = 2048;
pub static BUF: LazyLock<Box<[u8; 2048]>> = LazyLock::new(|| Box::new([0u8; 2048]));

pub fn to_string(size: i32) -> Option<String> {
    if size == 0 {
        return None;
    }
    str::from_utf8(&BUF[0..size as usize])
        .ok()
        .map(|s| s.to_string())
}
pub fn to_bytes(size: i32) -> Option<Vec<u8>> {
    if size == 0 {
        return None;
    }
    Some(BUF[0..size as usize].as_ref().to_owned())
}

pub fn handle_values(count_len: i64) -> Vec<Vec<u8>> {
    let (count, len) = i64_to_i32x2(count_len);
    let data = &BUF[0..len as usize];
    let mut out = Vec::with_capacity(count as usize);
    for bytes in split_u8_nul_utf8(data) {
        out.push(bytes.to_vec());
    }
    out
}

fn split_u8_nul_utf8(utf8_src: &[u8]) -> Vec<&[u8]> {
    let mut out = Vec::new();
    let mut start_index: usize = 0;
    for (i, n) in utf8_src.iter().enumerate() {
        if *n == b'\0' {
            let t = &utf8_src[start_index..i];
            out.push(t);
            start_index = i + 1; // skip NUL byte
        }
    }
    out
}
pub fn i64_to_i32x2(n: i64) -> (i32, i32) {
    (
        (n >> 32) as i32, //upper count
        n as i32,         //lower len
    )
}

pub fn eof_size(n: i64) -> (bool, i32) {
    (
        if (n >> 32) as i32 == 1 { true } else { false }, //upper
        n as i32,                                         //lower len
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_i64() {
        let (a, b) = i64_to_i32x2(2 << 32 | 28);
        assert_eq!((a, b), (2, 28));
    }
}
