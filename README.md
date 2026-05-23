# CryptoPals Challenges in Rust

This repository contains my personal, modular solutions to the famous [Cryptopals Cryptography Challenges](https://cryptopals.com/), written from scratch in high-quality, idiomatic Rust.

---

## 🛠️ Project Structure
To maintain a clean and extensible layout as we tackle more exercises, the repository is organized by subfolders for each individual challenge or topic:

```text
CryptoPals_Challenges/
├── README.md
├── .gitignore
└── 01-hex_to_base64/              # Set 1, Challenge 1: Hex to Base64
    ├── Cargo.toml
    ├── Cargo.lock
    ├── architecture_diagram.md
    └── src/
        ├── lib.rs              # Core library
        ├── hex.rs              # Scratch implementation of Hex codecs
        ├── base64.rs           # Scratch implementation of Base64 codecs
        └── main.rs             # Interactive CLI application
```

---

## ⚡ Challenge 1: Hex to Base64 Converter
The first challenge implements a fully dependency-free Hex and Base64 encoder and decoder adhering strictly to the **Cryptopals Rule** (operating entirely on raw bytes in transit).

### Key Features
1. **Interactive CLI with Line Editing**: Utilizes `rustyline` for a modern CLI experience, supporting cursor navigation with Arrow/Home/End keys, backspace, and input history.
2. **Automatic Plaintext Preview**: Detects if input bytes form valid, printable ASCII/UTF-8 and prints a preview representation of the plaintext string.
3. **Flexible Invocation**: Supports both passing the hex string directly as a command-line argument or running in interactive mode.
4. **10/10 Code Quality**: Self-documenting, idiomatic, boilerplate-free Rust implementation.

---

## 🚀 Getting Started

### Prerequisites
Make sure you have Rust and Cargo installed:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Running the Hex to Base64 Utility
1. Navigate to the challenge directory:
   ```bash
   cd 01-hex_to_base64
   ```
2. Run the application:
   ```bash
   cargo run
   ```
3. (Optional) Run with direct arguments:
   ```bash
   cargo run -- 48656c6c6f
   ```

### Running Tests
To verify implementation correctness:
```bash
cd 01-hex_to_base64
cargo test
```
