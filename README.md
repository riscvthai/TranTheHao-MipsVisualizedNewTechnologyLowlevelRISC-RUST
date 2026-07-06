<<<<<<< HEAD
# TranTheHao MIPS Visualized New Technology Low-level RISC Rust

## Objective

Visualize and explain MIPS `JAL` control flow using Rust, with a RARS-style execution model.
=======
# Trần Thế Hảo — MIPS JAL RARS Full Visual System v2 FIXED

## Objective

Fixed integrated package for:

```text
PNG/JPG charts
+ Mermaid diagrams
+ Active HTML simulator
+ Business model layer
+ GitHub-ready repository structure
```

## Fixed issues

- HTML no longer depends on fragile relative chart paths for preview.
- PNG charts are embedded into `index.html` as base64.
- PNG and JPG files still exist in `assets/charts/`.
- Mermaid data is injected through JSON-safe content, not unsafe JS template strings.
- Mermaid source is always visible even if CDN rendering fails.
- Active simulator is included in the same HTML.

## Main entry

Open:

```text
index.html
```

or:

```text
visualizer/full-system.html
```

## Structure

```text
assets/charts/
├── Tran-The-Hao_System-Low-Level-Active-Architecture.png
├── Tran-The-Hao_System-Low-Level-Active-Architecture.jpg
├── Tran-The-Hao_MIPS-JAL-Nested-Execution-Trace-Chart.png
└── Tran-The-Hao_MIPS-JAL-Nested-Execution-Trace-Chart.jpg

assets/diagrams/
├── 01-System-Low-Level-Active-Model.mmd
├── 02-MIPS-JAL-Mechanism.mmd
├── 03-Nested-JAL-Return-Address-Hazard.mmd
├── 04-Execution-Trace-jal-nested-call.mmd
└── 05-Business-System-Model.mmd

examples/
└── jal_nested_call.asm

visualizer/
├── full-system.html
└── business.html
```

## Push to GitHub

Run:

```bash
git init
git branch -M main
git remote add origin https://github.com/riscvthai/TranTheHao-MipsVisualizedNewTechnologyLowlevelRISC-RUST.git
git add .
git commit -m "Add fixed full visual system model"
git push -u origin main
```
>>>>>>> 227f056 (Add fixed full visual system model)

## Author

Trần Thế Hảo
<<<<<<< HEAD

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
=======
>>>>>>> 227f056 (Add fixed full visual system model)
