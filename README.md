# TranTheHao MIPS Visualized New Technology Low-level RISC Rust

## Objective

Visualize and explain MIPS `JAL` control flow using Rust, with a RARS-style execution model.

## Author

Trần Thế Hảo

## Core Focus

- MIPS `JAL` instruction
- Program Counter movement
- `$ra` return address update
- RARS-style execution flow
- Low-level RISC visualization
- Rust implementation skeleton

## Quick Start

```bash
cargo run
```

You can also provide custom addresses:

```bash
cargo run -- 0x00400000 0x00400020
```

## Project Structure

```text
.
├── Cargo.toml
├── README.md
├── src/
│   └── main.rs
├── docs/
│   ├── Architecture.md
│   └── MIPS-JAL-RARS-Flow.md
├── examples/
│   └── jal_basic.asm
└── assets/
    └── diagrams/
```

## Status

Initial skeleton repository.
