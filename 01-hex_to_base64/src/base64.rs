/// Converts a slice of raw bytes to its base64‑encoded string representation.
pub fn bytes_to_base64(input: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(((input.len() + 2) / 3) * 4);
    
    for chunk in input.chunks(3) {
        let bits = (chunk[0] as u32) << 16
            | (*chunk.get(1).unwrap_or(&0) as u32) << 8
            | (*chunk.get(2).unwrap_or(&0) as u32);
            
        out.push(ALPHABET[((bits >> 18) & 0x3F) as usize] as char);
        out.push(ALPHABET[((bits >> 12) & 0x3F) as usize] as char);
        out.push(if chunk.len() > 1 { ALPHABET[((bits >> 6) & 0x3F) as usize] as char } else { '=' });
        out.push(if chunk.len() > 2 { ALPHABET[(bits & 0x3F) as usize] as char } else { '=' });
    }
    out
}

/// Converts a base64 string to a vector of raw bytes.
/// Returns an error if the string is not a valid base64‑encoded representation.
pub fn base64_to_bytes(encoded: &str) -> Result<Vec<u8>, String> {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut rev = [255u8; 256];
    for (i, &c) in ALPHABET.iter().enumerate() { rev[c as usize] = i as u8; }

    let cleaned: Vec<u8> = encoded.bytes().filter(|b| !b.is_ascii_whitespace()).collect();
    if cleaned.len() % 4 != 0 { return Err("Base64 length must be a multiple of 4".into()); }

    let mut out = Vec::with_capacity((cleaned.len() / 4) * 3);
    for quad in cleaned.chunks(4) {
        let mut vals = [0u32; 4];
        let mut pad = 0;
        for (i, &b) in quad.iter().enumerate() {
            if b == b'=' { pad += 1; }
            else if pad > 0 { return Err("Unexpected character after padding".into()); }
            else if rev[b as usize] == 255 { return Err(format!("Invalid base64 character: {}", b as char)); }
            else { vals[i] = rev[b as usize] as u32; }
        }
        if pad > 2 { return Err("Too much base64 padding".into()); }
        
        let bits = (vals[0] << 18) | (vals[1] << 12) | (vals[2] << 6) | vals[3];
        out.push((bits >> 16) as u8);
        if pad < 2 { out.push((bits >> 8) as u8); }
        if pad < 1 { out.push(bits as u8); }
    }
    Ok(out)
}

