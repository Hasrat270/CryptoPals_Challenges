pub mod base64;
pub mod hex;

/// Converts a hex-encoded string to a base64-encoded string.
/// 
/// This strictly adheres to the **Cryptopals Rule**:
/// 1. Decode the hex string into raw bytes.
/// 2. Perform any operations on raw bytes (here, none, just passing the bytes along).
/// 3. Encode the raw bytes to base64.
pub fn hex_to_base64(hex_str: &str) -> Result<String, String> {
    let raw_bytes = hex::hex_to_bytes(hex_str)?;
    Ok(base64::bytes_to_base64(&raw_bytes))
}

