pub mod instruction;
pub mod trace;

use std::collections::HashSet;

use crate::instruction::{ArithmeticOp, Instruction};
use crate::trace::{TraceEntry, Tracer};

#[derive(Debug, PartialEq)]
pub enum CpuFault {
    Loop(usize),
}

#[derive(Debug)]
pub struct HandHeld {
    r0: i64,
    pc: usize,
    ran_pcs: HashSet<usize>,
    tracer: Tracer,
    tracing_enabled: bool,
}

impl HandHeld {
    // TODO: add enum or options struct
    pub fn new(tracing_enabled: bool) -> Self {
        Self {
            r0: 0i64,
            pc: 0usize,
            ran_pcs: HashSet::default(),
            tracer: Tracer::new(),
            tracing_enabled,
        }
    }

    pub fn run_code(&mut self, code: &[&str]) -> Result<(), CpuFault> {
        let mut clock = 0u128;
        loop {
            clock += 1;
            let inst = match code.get(self.pc) {
                Some(inst) => Instruction::decode(inst),
                None => break,
            };

            if !self.ran_pcs.insert(self.pc) {
                return Err(CpuFault::Loop(self.pc));
            }

            match inst {
                Instruction::Nop => {
                    self.pc += 1;
                }
                Instruction::AccImmediate(op, imm) => {
                    match op {
                        ArithmeticOp::Plus => {
                            self.r0 += imm as i64;
                        }
                        ArithmeticOp::Minus => {
                            self.r0 -= imm as i64;
                        }
                    };
                    self.pc += 1;
                }
                Instruction::JmpRelative(op, imm) => match op {
                    ArithmeticOp::Plus => {
                        self.pc += imm;
                    }
                    ArithmeticOp::Minus => {
                        self.pc -= imm;
                    }
                },
            }

            if self.tracing_enabled {
                self.tracer
                    .log(TraceEntry::new(clock, self.r0, self.pc, inst));
            }
        }
        Ok(())
    }

    pub fn dump_registers(&self) {
        println!("r0: {} pc: {}", self.r0, self.pc);
    }

    pub fn serialize_trace(&self) -> String {
        self.tracer.serialize()
    }

    pub fn reset(&mut self) {
        self.r0 = 0;
        self.pc = 0;
        self.ran_pcs = HashSet::default();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_run_code() {
        let mut handheld = HandHeld::new(false);
        handheld.run_code(&["jmp +2", "acc +5", "acc +10"]).unwrap();

        assert_eq!(10, handheld.r0);
        assert_eq!(3, handheld.pc);
    }

    #[test]
    fn test_run_code_loop_detect() {
        let mut handheld = HandHeld::new(false);
        let ret = handheld.run_code(&["jmp +2", "acc +5", "jmp -2"]);

        assert_eq!(CpuFault::Loop(0), ret.unwrap_err());
        assert_eq!(0, handheld.r0);
    }

    #[test]
    fn test_reset() {
        let mut handheld = HandHeld::new(false);
        handheld.run_code(&["jmp +2", "acc +5", "acc +10"]).unwrap();

        assert_eq!(10, handheld.r0);
        assert_eq!(3, handheld.pc);

        handheld.reset();

        assert_eq!(0, handheld.r0);
        assert_eq!(0, handheld.pc);
    }

    #[test]
    fn test_with_tracing() {
        let mut handheld = HandHeld::new(true);
        handheld.run_code(&["jmp +2", "acc +5", "acc +10"]).unwrap();

        let expected = r#"1 0 2 jmp +2
2 10 3 acc +10"#;

        assert_eq!(10, handheld.r0);
        assert_eq!(3, handheld.pc);

        assert_eq!(expected, handheld.serialize_trace());
    }
}
