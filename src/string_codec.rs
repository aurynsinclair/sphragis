//! Provides conversion utilities between strings and byte sequences.


use eyre::{
    eyre,
    Result,
};
use zeroize::Zeroizing;
use base64::{
    Engine,
    engine::general_purpose::STANDARD as b64,
};
use hex;


/// Renders a BIP-39 passphrase.
/// Base64-eoncodes the given bytes and returns the output as Zeroizing-wrapped string.
pub fn passphrase_from_key(key: Zeroizing<[u8; 32]>) -> Zeroizing<String> {
    let mut output_buf = Zeroizing::new(String::new());
    b64.encode_string(key, &mut output_buf);
    output_buf
}


/// Renders a salt string to write in a config file.
/// Simply Base64-eoncodes the given bytes and returns the string.
pub fn base64_string_from_salt(salt: &[u8]) -> String {
    b64.encode(salt)
}


/// Parses Base64 encoded string into bytes.
pub fn parse_base64_str(s: &str) -> Result<Vec<u8>> {
    b64.decode(s).map_err(|e| eyre!("{e:?}"))
}


/// Simply hex-eoncodes the given bytes and returns the string (for debugging).
pub fn hex_string_from_bytes(bytes: &[u8]) -> String {
    hex::encode(bytes)
}


/// Converts the string into UTF-8 bytes.
pub fn into_utf8_bytes(s: &str) -> Vec<u8> {
    s.as_bytes().to_vec()
}