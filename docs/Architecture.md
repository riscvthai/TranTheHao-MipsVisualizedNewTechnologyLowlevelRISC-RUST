# Architecture

## Objective

Build a minimal Rust model that explains how a MIPS `JAL` instruction changes CPU control flow.

## Core Model

```text
Before execution
PC points to the `jal` instruction
$ra still holds the previous return address, or zero at program start

System state before JAL
1. The caller is still active
2. The callee has not been entered yet
3. The next instruction address is not stored in $ra yet

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

In other words, the system starts from the current CPU state, records the address of the next instruction, then transfers control to the target label.

This means the important "before" state is the caller-side snapshot: the PC still belongs to the `jal` instruction, while `$ra` is only updated after decode and link.

This skeleton intentionally keeps the model small so the control-flow mechanism is easy to verify.
