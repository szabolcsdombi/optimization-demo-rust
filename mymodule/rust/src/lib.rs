use sha1::Sha1;
use sha1::Digest;

pub fn sec_websocket_accept(key: &str) -> String {
    let mut concat_key = String::with_capacity(key.len() + 36);
    concat_key.push_str(&key[..]);
    concat_key.push_str("258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
    let hash = Sha1::digest(concat_key.as_bytes());
    base64::encode(hash.as_slice())
}

#[no_mangle]
pub extern fn rust_accept(key: *const u8, result: *mut u8) {
    unsafe {
        let source = std::slice::from_raw_parts(key, 24);
        let source_str = std::str::from_utf8_unchecked(source);
        let modified_str = sec_websocket_accept(source_str);
        let dest = std::slice::from_raw_parts_mut(result, 28);
        dest[..28].copy_from_slice(modified_str.as_bytes());
    }
}
