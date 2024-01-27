#[cfg(feature = "encode")]
/// Converts a hash to a hex string
pub fn bytes_to_hex(bytes: impl AsRef<[u8]>) -> String {
    let mut s = String::new();
    let table = b"0123456789abcdef";
    for &b in bytes.as_ref().iter() {
        s.push(table[(b >> 4) as usize] as char);
        s.push(table[(b & 0xf) as usize] as char);
    }
    s
}


#[cfg(feature = "encode")]
/// Utility trait for converting hashes to hex strings
pub trait Encodable {
    /// Converts a hash to a hex string
    fn to_hex_string(&self) -> String;
}

#[cfg(feature = "encode")]
impl Encodable for Vec<u8> {
    fn to_hex_string(&self) -> String { bytes_to_hex(self) }
}

#[cfg(feature = "encode")]
impl Encodable for &[u8] {
    fn to_hex_string(&self) -> String { bytes_to_hex(self) }
}


