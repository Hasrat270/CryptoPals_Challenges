use std::env;
use rustyline::DefaultEditor;
use cryptopals::hex::hex_to_bytes;
use cryptopals::base64::bytes_to_base64;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    
    let input_hex = if arguments.len() > 1 {
        arguments[1].trim().to_string()
    } else {
        // Interactively ask for input
        println!("\x1b[1;36m================================================================");
        println!("                     HEX TO BASE64 CONVERTER                    ");
        println!("================================================================\x1b[0m");
        println!("\x1b[1;33mEnter a Hex string to convert (or press Enter for default challenge):\x1b[0m");
        
        let mut line_editor = DefaultEditor::new().unwrap();
        let readline_result = line_editor.readline("> ");
        
        match readline_result {
            Ok(input_line) => {
                let trimmed_line = input_line.trim().to_string();
                if trimmed_line.is_empty() {
                    "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string()
                } else {
                    trimmed_line
                }
            }
            Err(_) => {
                // If user hits Ctrl-C or Ctrl-D, fallback to the default challenge string
                "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string()
            }
        }
    };

    // If arguments were provided, print the header here since it wasn't printed above
    if arguments.len() > 1 {
        println!("\x1b[1;36m================================================================");
        println!("                     HEX TO BASE64 CONVERTER                    ");
        println!("================================================================\x1b[0m");
    }

    println!("\n\x1b[1;33m[Input Hex]:\x1b[0m");
    println!("  {}", input_hex);
    
    match hex_to_bytes(&input_hex) {
        Ok(raw_bytes) => {
            // Check if the raw bytes form a valid, printable ASCII/UTF-8 text
            if let Ok(plaintext) = String::from_utf8(raw_bytes.clone()) {
                if plaintext.chars().all(|c| c.is_ascii_graphic() || c.is_ascii_whitespace()) {
                    println!("\n\x1b[1;35m[Decoded Plaintext]:\x1b[0m");
                    println!("  \"{}\"", plaintext);
                }
            }
            
            let base64 = bytes_to_base64(&raw_bytes);
            println!("\n\x1b[1;32m[Output Base64]:\x1b[0m");
            println!("  {}", base64);
            println!("\n\x1b[1;32m✔ SUCCESS: Conversion completed successfully!\x1b[0m");
        }
        Err(error_message) => {
            println!("\n\x1b[1;31m✘ ERROR: {}\x1b[0m", error_message);
        }
    }
    
    println!("\x1b[1;36m================================================================\x1b[0m");
}
