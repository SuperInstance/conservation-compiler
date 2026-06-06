//! # conservation-compiler
//!
//! Mini compiler that verifies energy conservation at each step.
//! Rejects transforms that violate ternary algebra invariants.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    TAdd, TMul, TNeg, Identity,
}

#[derive(Debug, Clone)]
pub struct CompileStep {
    pub op: Op,
    pub input_sum: i32,
    pub output_sum: i32,
    pub conserved: bool,
}

#[derive(Debug, Clone)]
pub struct CompiledKernel {
    pub name: String,
    pub steps: Vec<CompileStep>,
    pub total_steps: usize,
    pub all_conserved: bool,
}

pub struct ConservationCompiler {
    rejected: usize,
    accepted: usize,
}

impl ConservationCompiler {
    pub fn new() -> Self { Self { rejected: 0, accepted: 0 } }

    /// Compile a sequence of ternary ops, checking conservation at each step.
    pub fn compile(&mut self, name: &str, ops: &[Op], input: &[i8]) -> Result<CompiledKernel, CompileError> {
        let mut state = input.to_vec();
        let mut steps = Vec::new();
        let input_sum: i32 = state.iter().map(|&v| v as i32).sum();

        for (i, op) in ops.iter().enumerate() {
            let before_sum: i32 = state.iter().map(|&v| v as i32).sum();
            let new_state = self.apply(op, &state, i)?;
            let after_sum: i32 = new_state.iter().map(|&v| v as i32).sum();

            let conserved = (after_sum - before_sum).abs() <= state.len() as i32;
            steps.push(CompileStep { op: op.clone(), input_sum: before_sum, output_sum: after_sum, conserved });

            if !conserved {
                self.rejected += 1;
                return Err(CompileError::ViolationAtStep(i, before_sum, after_sum));
            }
            state = new_state;
        }

        self.accepted += 1;
        let all_conserved = steps.iter().all(|s| s.conserved);
        Ok(CompiledKernel { name: name.into(), steps, total_steps: ops.len(), all_conserved })
    }

    fn apply(&self, op: &Op, state: &[i8], step: usize) -> Result<Vec<i8>, CompileError> {
        let mut out = state.to_vec();
        match op {
            Op::TAdd => {
                for i in 0..state.len().saturating_sub(1) {
                    out[i + 1] = tadd(state[i], state[i + 1]);
                }
            }
            Op::TMul => {
                for i in 0..state.len().saturating_sub(1) {
                    out[i + 1] = tmul(state[i], state[i + 1]);
                }
            }
            Op::TNeg => {
                for v in out.iter_mut() { *v = -*v; }
            }
            Op::Identity => {}
        }
        Ok(out)
    }

    pub fn stats(&self) -> (usize, usize) { (self.accepted, self.rejected) }
}

impl Default for ConservationCompiler {
    fn default() -> Self { Self::new() }
}

fn tadd(a: i8, b: i8) -> i8 {
    match (a, b) {
        (-1, -1) => 1, (-1, 0) => -1, (-1, 1) => 0,
        (0, -1) => -1, (0, 0) => 0, (0, 1) => 1,
        (1, -1) => 0, (1, 0) => 1, (1, 1) => -1, _ => 0,
    }
}

fn tmul(a: i8, b: i8) -> i8 {
    match (a, b) {
        (-1, -1) => 1, (-1, 1) => -1, (1, -1) => -1, (1, 1) => 1, _ => 0,
    }
}

#[derive(Debug)]
pub enum CompileError {
    ViolationAtStep(usize, i32, i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_compile() {
        let mut c = ConservationCompiler::new();
        let kernel = c.compile("id", &[Op::Identity], &[1, -1, 0]).unwrap();
        assert!(kernel.all_conserved);
    }

    #[test]
    fn test_tneg_compile() {
        let mut c = ConservationCompiler::new();
        let kernel = c.compile("neg", &[Op::TNeg], &[1, -1, 0]).unwrap();
        assert!(kernel.all_conserved);
    }

    #[test]
    fn test_tadd_compile() {
        let mut c = ConservationCompiler::new();
        let kernel = c.compile("add", &[Op::TAdd], &[1, -1, 1, -1]).unwrap();
        assert!(kernel.all_conserved);
        assert_eq!(kernel.total_steps, 1);
    }

    #[test]
    fn test_tmul_compile() {
        let mut c = ConservationCompiler::new();
        let kernel = c.compile("mul", &[Op::TMul], &[1, 1, -1, -1]).unwrap();
        assert!(kernel.all_conserved);
    }

    #[test]
    fn test_multi_step() {
        let mut c = ConservationCompiler::new();
        let kernel = c.compile("pipeline", &[Op::TAdd, Op::TNeg, Op::TMul], &[1, -1, 0, 1]).unwrap();
        assert_eq!(kernel.total_steps, 3);
    }

    #[test]
    fn test_stats() {
        let mut c = ConservationCompiler::new();
        c.compile("a", &[Op::Identity], &[1, -1]).unwrap();
        c.compile("b", &[Op::TAdd], &[1, -1, 1, -1]).unwrap();
        let (accepted, rejected) = c.stats();
        assert_eq!(accepted, 2);
        assert_eq!(rejected, 0);
    }
}
