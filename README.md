# conservation-compiler

A **conservation-aware mini compiler** that verifies energy/information conservation at each compilation step. It operates on ternary algebra (balanced ternary: {-1, 0, +1}) and rejects any program transformation that violates conservation invariants — the sum of the state vector must remain bounded across every operation.

## Why It Matters

This crate is an experimental testbed for **verified compilation**: instead of trusting that compiler passes preserve semantics, every single transform is checked against a conservation law. This matters for:

- **Formal methods** — verified compilers like CompCert prove semantic preservation; conservation laws are a lighter-weight invariant that catches real bugs
- **Quantum computing compilation** — unitary transforms must preserve norm (∑|ψᵢ|² = 1), analogous to this crate's conservation checks
- **Physics simulations** — symplectic integrators must conserve energy; this crate demonstrates the pattern in a compilation context
- **Ternary computing** — balanced ternary ({-1, 0, +1}) is the most efficient integer representation for theoretical computation

## How It Works

### Ternary Algebra

Operations use balanced ternary values $v \in \{-1, 0, +1\}$ with custom arithmetic:

**Addition** (mod 3 with balanced representation):

$$a \oplus b = \begin{cases} -1 \oplus -1 = +1 & \text{(wrap)} \\ -1 \oplus 0 = -1 & \\ 0 \oplus 0 = 0 & \\ +1 \oplus +1 = -1 & \text{(wrap)} \\ \text{etc.} & \end{cases}$$

**Multiplication** (standard sign rules, result clamped to ternary):

$$a \otimes b = \begin{cases} +1 & \text{if } a, b \in \{-1, +1\} \text{ and same sign} \\ -1 & \text{if } a, b \in \{-1, +1\} \text{ and opposite sign} \\ 0 & \text{if either is } 0 \end{cases}$$

### Conservation Law

For a state vector $\mathbf{s} = [s_1, s_2, \ldots, s_n]$ where $s_i \in \{-1, 0, +1\}$, define:

$$\Sigma(\mathbf{s}) = \sum_{i=1}^{n} s_i$$

After applying any compile step $T$:

$$|\Sigma(T(\mathbf{s})) - \Sigma(\mathbf{s})| \leq n$$

where $n$ is the state length. If this bound is violated, the transform is **rejected**.

### Compilation Pipeline

Each `CompileStep` records:

```rust
pub struct CompileStep {
    pub op: Op,
    pub input_sum: i32,   // Σ before
    pub output_sum: i32,   // Σ after
    pub conserved: bool,
}
```

The compiler tracks accept/reject statistics:

```rust
pub fn stats(&self) -> (usize, usize) // (accepted, rejected)
```

### Operations

| Op | Semantics | Conservation Check |
|----|-----------|-------------------|
| `Identity` | No-op | Always conserved |
| `TNeg` | Negate all elements | Sum flips sign; |ΔΣ| ≤ n ✓ |
| `TAdd` | Adjacent ternary add | Sum changes by wrapping |
| `TMul` | Adjacent ternary multiply | Sum changes by clamping |

### Complexity Analysis

| Operation | Time | Space |
|-----------|------|-------|
| `compile(name, ops, input)` | O(|ops| × n) | O(n) |
| `apply(op, state)` | O(n) | O(n) |
| Conservation check | O(n) | O(1) |

Where n = state vector length.

## Quick Start

```rust
use conservation_compiler::{ConservationCompiler, Op};

let mut compiler = ConservationCompiler::new();

// Compile a pipeline of ternary ops
let kernel = compiler.compile(
    "pipeline",
    &[Op::TAdd, Op::TNeg, Op::TMul],
    &[1, -1, 0, 1],
).unwrap();

assert!(kernel.all_conserved);
assert_eq!(kernel.total_steps, 3);

let (accepted, rejected) = compiler.stats();
```

## API

| Type / Method | Description |
|---------------|-------------|
| `ConservationCompiler::new()` | Create compiler |
| `compile(name, ops, input) → Result<CompiledKernel, CompileError>` | Compile with verification |
| `stats() → (usize, usize)` | (accepted, rejected) count |
| `Op::Identity / TAdd / TMul / TNeg` | Ternary operations |
| `CompileError::ViolationAtStep(step, before, after)` | Conservation violation |

## Architecture Notes

The **γ + η = C** link is the entire purpose of this crate: the ternary operations (γ) transform the state vector, while the conservation check (η) validates that the transform preserves the bounded-sum invariant. Together they enforce the invariant C — every accepted kernel provably satisfies |Σ_output − Σ_input| ≤ n. This is the **conservation compiler principle**: compilation is correct-by-construction because each step is verified, not just the final result.

## References

- Knuth, D. E. (2008). *The Art of Computer Programming, Vol. 2: Seminumerical Algorithms,* 3rd ed., Section 4.1: Positional Number Systems. (Balanced ternary.)
- Leroy, X. (2009). *Formal Verification of a Realistic Compiler.* Communications of the ACM, 52(7), 107–115. (CompCert.)
- Nielsen, M. A., & Chuang, I. L. (2010). *Quantum Computation and Quantum Information.* (Unitary conservation = norm preservation.)
- Toffoli, T. (1982). *Physics and Computation.* IJTP, 21(3–4), 165–175. (Conservative logic gates.)
- Feynman, R. P. (1985). *Quantum Mechanical Computers.* Optics News, 11(2), 11–20. (Reversible/conservative computation.)

## License

MIT
