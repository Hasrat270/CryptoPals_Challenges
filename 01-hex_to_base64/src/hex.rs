/// Converts a hexadecimal string to a vector of raw bytes.
/// Returns an error if the hex string has an odd length or contains invalid hex characters.
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, String> {
    if hex.len() % 2 != 0 {
        return Err("Hex string must have an even length".to_string());
    }
    let mut bytes = Vec::with_capacity(hex.len() / 2);
    let mut chars = hex.chars();
    while let (Some(first_char), Some(second_char)) = (chars.next(), chars.next()) {
        let high_nibble = first_char
            .to_digit(16)
            .ok_or_else(|| format!("Invalid hex character: {}", first_char))? as u8;
        let low_nibble = second_char
            .to_digit(16)
            .ok_or_else(|| format!("Invalid hex character: {}", second_char))? as u8;
        bytes.push((high_nibble << 4) | low_nibble);
    }
    Ok(bytes)
}

/// Converts a slice of raw bytes to a hexadecimal string representation.
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";
    let mut hex_string = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        hex_string.push(HEX_CHARS[(byte >> 4) as usize] as char);
        hex_string.push(HEX_CHARS[(byte & 0x0f) as usize] as char);
    }
    hex_string
}

