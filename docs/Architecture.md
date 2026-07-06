# Architecture

## Objective

Build a minimal Rust model that explains how a MIPS `JAL` instruction changes CPU control flow.

## System Overview

This project is a small CPU-flow simulator and explanation layer.

It has three visible layers:

1. Runtime logic in `src/main.rs`.
2. Explanatory documentation in `docs/`.
3. Example assembly input in `examples/`.

The runtime layer simulates a single `JAL` transition, while the docs describe what the machine is doing before and after that transition.

## Component Map

| Component | Location | What it does | System impact |
|---|---|---|---|
| `main()` | `src/main.rs` | Starts the demo, reads inputs, prints the current state, and runs the simulated jump | Defines the visible execution order for the whole program |
| `parse_address()` | `src/main.rs` | Converts command-line input into a `u32` address | Controls the initial PC and the jump target shown by the demo |
| `CpuState` | `src/main.rs` | Stores the simulated `pc` and `ra` registers | Represents the machine state being visualized |
| `execute_jal()` | `src/main.rs` | Applies the JAL rule: save return address, then jump | Mutates CPU state and models the hardware effect of `jal` |
| `README.md` | Project root | Gives quick-start usage and repo summary | Helps a user launch the demo correctly |
| `docs/Architecture.md` | `docs/Architecture.md` | Explains the architecture and execution model | Provides the mental model for the system |
| `docs/MIPS-JAL-RARS-Flow.md` | `docs/MIPS-JAL-RARS-Flow.md` | Describes the JAL flow in step form | Shows how control moves from caller to callee and back |
| `examples/jal_basic.asm` | `examples/jal_basic.asm` | Holds a tiny MIPS example | Supplies the kind of input the model is meant to explain |

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

## Runtime Sequence

The simulated system works in this order:

1. `main()` reads the initial PC and target address.
2. `CpuState` is created with that starting PC and `$ra = 0`.
3. The program prints the pre-JAL state so the caller context is visible.
4. `execute_jal()` stores `PC + 4` into `$ra`.
5. `execute_jal()` replaces `pc` with the target address.
6. The final state is printed so the impact of the jump is explicit.

This is the exact place where the system changes behavior: before `execute_jal()`, the machine is still in the caller. After it, control has moved to the callee target.

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

## What Changes Where

- `pc` changes from the address of the current `jal` instruction to the target label.
- `ra` changes from its previous value to `pc + 4`.
- The caller context becomes inactive once the jump is taken.
- The callee context becomes active immediately after the jump.
- The documentation files do not change runtime behavior; they only explain it.

This skeleton intentionally keeps the model small so the control-flow mechanism is easy to verify.
