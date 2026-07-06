# Architecture

## Objective

Build a minimal Rust model that explains how a MIPS `JAL` instruction changes CPU control flow.

## Core Model

```text
Instruction Input
→ Decode JAL
→ Save PC + 4 into $ra
→ Jump to target address
→ Continue execution from target
```

## Main Components

| Component | Role |
|---|---|
| `CpuState` | Stores simulated `pc` and `ra` values |
| `execute_jal()` | Applies the core JAL transition rule |
| `examples/` | Stores MIPS assembly examples |
| `docs/` | Stores explanation and visualized flow notes |

## Rule

For a MIPS `JAL target` instruction:

```text
$ra = PC + 4
PC  = target
```

This skeleton intentionally keeps the model small so the control-flow mechanism is easy to verify.
