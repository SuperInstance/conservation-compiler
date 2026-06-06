# conservation-compiler

Experiment: mini compiler that verifies energy conservation at each compilation step. Rejects transforms that violate ternary algebra invariants.

## Why This Matters

# conservation-compiler
Mini compiler that verifies energy conservation at each step.
Rejects transforms that violate ternary algebra invariants.

## The Five-Layer Stack

This crate is part of the **Oxide Stack** — a distributed GPU runtime built on five layers:

```
┌─────────────────┐
│  cudaclaw        │  Persistent GPU kernels, warp consensus, SmartCRDT
├─────────────────┤
│  cuda-oxide      │  Flux → MIR → Pliron → NVVM → PTX compiler
├─────────────────┤
│  flux-core       │  Bytecode VM + A2A agent protocol
├─────────────────┤
│  pincher         │  "Vector DB as runtime, LLM as compiler"
├─────────────────┤
│  open-parallel   │  Async runtime (tokio fork)
└─────────────────┘
```

The key insight: **ternary values {-1, 0, +1} map directly to GPU compute**. They pack 16× denser than FP32, enable XNOR+popcount matmul, and conservation laws become compile-time checks.

## Design

Every value in this crate follows **ternary algebra** (Z₃):

| Value | Meaning | GPU Analog |
|-------|---------|------------|
| +1 | Positive / Active / Healthy | Warp vote yes |
| 0 | Neutral / Pending / Balanced | Warp vote abstain |
| -1 | Negative / Failed / Overloaded | Warp vote no |

This isn't arbitrary — ternary is the natural encoding for:
1. **BitNet b1.58** (Microsoft) — ternary LLMs at 60% less power
2. **GPU warp voting** — hardware ballot returns ternary consensus
3. **Conservation laws** — {-1, 0, +1} preserves quantity

## Key Types

```rust
pub enum Op
pub struct CompileStep
pub struct CompiledKernel
pub struct ConservationCompiler
pub fn new
pub fn compile
pub fn stats
pub enum CompileError
```

## Usage

```toml
[dependencies]
conservation-compiler = "0.1.0"
```

```rust
use conservation_compiler::*;
// See src/lib.rs tests for complete working examples
```

## Testing

```bash
git clone https://github.com/SuperInstance/conservation-compiler.git
cd conservation-compiler
cargo test    # 6 tests
```

## Stats

| Metric | Value |
|--------|-------|
| Tests | 6 |
| Lines of Rust | 158 |
| Public API | 8 items |

## License

Apache-2.0
