/// Converts a hash to a hex string
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut s = String::new();
    let table = b"0123456789abcdef";
    for &b in bytes.iter() {
        s.push(table[(b >> 4) as usize] as char);
        s.push(table[(b & 0xf) as usize] as char);
    }
    s
}
