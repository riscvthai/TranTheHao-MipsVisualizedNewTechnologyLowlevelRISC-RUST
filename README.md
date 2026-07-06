# Trần Thế Hảo — MIPS JAL RARS Full Visual System (Rust & Web)

## Objective

Visualize and explain MIPS `JAL` control flow using Rust, with a RARS-style execution model. This repository provides an integrated package for active HTML simulation, low-level architectural diagrams, and a Rust execution skeleton.

### Fixed Issues (v2)
- HTML no longer depends on fragile relative chart paths for preview.
- PNG charts are embedded into `index.html` as base64.
- PNG and JPG files still exist in `assets/charts/`.
- Mermaid data is injected through JSON-safe content, not unsafe JS template strings.
- Mermaid source is always visible even if CDN rendering fails.
- Active simulator is included in the same HTML.

## Core Focus

- MIPS `JAL` instruction mechanics.
- Program Counter (PC) movement.
- `$ra` (Return Address) update logic.
- RARS-style execution flow simulation.
- Low-level RISC visualization (PNG/JPG + Mermaid).
- Rust implementation skeleton.

## Quick Start & Main Entry

### 1. Rust Simulator
To run the Rust-based execution model:
```bash
cargo run
