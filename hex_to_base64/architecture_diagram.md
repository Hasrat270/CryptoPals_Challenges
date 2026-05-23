# System Architecture Diagram

This document presents the architectural layout and execution data flow of the `cryptopals` hex-to-base64 conversion utility.

---

## 1. Project Crate Architecture
Our workspace is laid out as a modular Rust crate. The interactive binary driver ([src/main.rs](file:///home/hasrat/Documents/CryptoPals_Challenges/src/main.rs)) depends on our core library ([src/lib.rs](file:///home/hasrat/Documents/CryptoPals_Challenges/src/lib.rs)), which cleanly exports standalone encoding modules.

```mermaid
graph TD
    subgraph Binary Crate
        Main[src/main.rs <br/> CLI Driver]
    end

    subgraph Library Crate (cryptopals)
        Lib[src/lib.rs <br/> Crate Root]
        HexMod[src/hex.rs <br/> Hex Encoder/Decoder]
        B64Mod[src/base64.rs <br/> Base64 Encoder/Decoder]
    end

    %% Dependency Links %%
    Main -->|Imports| Lib
    Lib -->|Exposes| HexMod
    Lib -->|Exposes| B64Mod
```

---

## 2. Execution & Data Flow Diagram
This flowchart tracks the conversion pipeline, showing how input is processed step-by-step, how it respects the **Cryptopals Rule** (operating entirely on raw bytes in transit), and how printable plaintext is automatically detected for terminal display.

```mermaid
graph TD
    %% CLI Input Flow %%
    subgraph CLI ["CLI Layer (main.rs)"]
        Start([Start Program]) --> CheckArgs{CLI Args Provided?}
        CheckArgs -- Yes --> ArgInput[Use Hex String from Arguments]
        CheckArgs -- No --> Interactive[Prompt User via Rustyline]
        Interactive --> CheckEmpty{Input Empty?}
        CheckEmpty -- Yes --> DefaultHex[Fallback: Vanilla Ice Hex String]
        CheckEmpty -- No --> UserHex[Use Typed Hex String]
    end

    %% Decoding Flow %%
    subgraph HexLib ["Hex Module (hex.rs)"]
        ArgInput & DefaultHex & UserHex --> HexToBytes[hex_to_bytes]
        HexToBytes -->|Validates Even Length & Chars| RawBytes[Vector of Raw Bytes: Vec<span>u8</span>]
        HexToBytes -.->|Invalid Hex Error| ErrorExit[Display Error & Exit]
    end

    %% Plaintext Preview Flow %%
    subgraph PlaintextPreview ["Plaintext Preview Detector"]
        RawBytes --> TryUtf8{Is Valid UTF-8 & Printable?}
        TryUtf8 -- Yes --> DisplayText[Display Decoded Plaintext Preview]
        TryUtf8 -- No --> SkipPreview[Skip Plaintext Preview]
    end

    %% Encoding Flow %%
    subgraph B64Lib ["Base64 Module (base64.rs)"]
        RawBytes --> BytesToB64[bytes_to_base64]
        BytesToB64 --> B64Output[Base64 String Output]
    end

    %% Output and End %%
    DisplayText & SkipPreview & B64Output --> DisplayResults[Print Output and Success Banner]
    DisplayResults --> EndProgram([End Program])

    %% Styling %%
    classDef cli fill:#e1f5fe,stroke:#03a9f4,stroke-width:2px;
    classDef hex fill:#f5f5f5,stroke:#9e9e9e,stroke-width:2px;
    classDef preview fill:#fce4ec,stroke:#ec407a,stroke-width:2px;
    classDef b64 fill:#f1f8e9,stroke:#7cb342,stroke-width:2px;
    
    class Start,CheckArgs,ArgInput,Interactive,CheckEmpty,DefaultHex,UserHex,DisplayResults,EndProgram cli;
    class HexToBytes,RawBytes,ErrorExit hex;
    class TryUtf8,DisplayText,SkipPreview preview;
    class BytesToB64,B64Output b64;
```

---

## 3. Key Architectural Principles

1. **The Cryptopals Rule (Strict Encapsulation)**:
   The library never performs "string-to-string" conversions directly. Hex is parsed down to raw byte arrays (`Vec<u8>`), which serve as the universal data currency. The Base64 module only serializes raw bytes back to string format.
2. **Zero External Cryptographic Dependencies**:
   Both encoding and decoding algorithms are written purely in native safe Rust code to maintain full control over the execution mechanics.
3. **Robust Separation of Concerns**:
   - `hex.rs` is solely concerned with Base16 serializations.
   - `base64.rs` is solely concerned with Base64 serializations.
   - `main.rs` manages user experience, CLI interfaces, and terminal decoration.
