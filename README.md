# 🌌 Nebula
[![codecov](https://codecov.io/gh/TStrecha/nebula/graph/badge.svg?token=HY9PPYTZWV)](https://codecov.io/gh/TStrecha/nebula)

**Nebula** is an experimental project focused on low-level systems development, written in Rust.

### 🔧 NVM – Nebula Virtual Machine

`nvm` is a simple 16-bit virtual machine inspired by the x86 architecture.

#### ✨ Highlights

* 16-bit registers and instruction set
* Basic instructions like `MOV`, `PUSH`, `POP`, and `NOOP`
* ModR/M byte decoding
* Register and memory operand support
* Little-endian encoding for multi-byte values

#### 🚀 Build & Test

```bash
cargo build
cargo test
```

#### 🧪 Quick Example

```rust
machine.load_program_bytes(&[0xB8, 0x34, 0x12]); // MOV AX, 0x1234
machine.step();
```

#### ▶️ Running a binary file

You can run a binary program file using:

```bash
cargo run --bin nvm {binary file}
```

---

For now, this project serves as a learning tool and playground for experimenting with instruction decoding and emulation.
