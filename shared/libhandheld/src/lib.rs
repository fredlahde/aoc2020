#[derive(Debug, PartialEq)]
enum ArithmeticOp {
    PLUS,
    MINUS,
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Nop,
    AccImmediate(ArithmeticOp, usize),
    JmpRelative(ArithmeticOp, usize),
}

impl Instruction {
    // TODO: do not panic
    fn decode(in_string: &str) -> Self {
        if in_string.starts_with("nop") {
            return Instruction::Nop;
        }
        let mut split = in_string.split(" ");
        let inst = split.next().unwrap();

        let op = split.next().unwrap().as_bytes();
        let op_arg = &op[0];
        let op_arg = match *op_arg as char {
            '+' => ArithmeticOp::PLUS,
            '-' => ArithmeticOp::MINUS,
            _ => panic!("invalid op arg"),
        };

        let op_imm = &op[1..];
        let op_imm = std::str::from_utf8(op_imm).unwrap();
        let op_imm: usize = op_imm.parse().unwrap();

        match inst {
            "acc" => Instruction::AccImmediate(op_arg, op_imm),
            "jmp" => Instruction::JmpRelative(op_arg, op_imm),
            _ => panic!("SIGILL"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum CpuFault {
    Loop(usize),
}

#[derive(Debug, Default)]
pub struct HandHeld {
    r0: u64,
    pc: usize,
    ran_pcs: Vec<usize>,
}

impl HandHeld {
    pub fn run_code(&mut self, code: &[&str]) -> Result<(), CpuFault> {
        loop {
            let inst = match code.get(self.pc) {
                Some(inst) => Instruction::decode(inst),
                None => break,
            };

            //println!("running {:?} pc {}", inst, self.pc);

            if self.ran_pcs.contains(&self.pc) {
                return Err(CpuFault::Loop(self.pc));
            }
            self.ran_pcs.push(self.pc);

            match inst {
                Instruction::Nop => {
                    self.pc += 1;
                }
                Instruction::AccImmediate(op, imm) => {
                    match op {
                        ArithmeticOp::PLUS => {
                            self.r0 += imm as u64;
                        },
                        ArithmeticOp::MINUS => {
                            self.r0 -= imm as u64;
                        }
                    };
                    self.pc += 1;
                }
                Instruction::JmpRelative(op, imm) => match op {
                    ArithmeticOp::PLUS => {
                        self.pc += imm;
                    }
                    ArithmeticOp::MINUS => {
                        self.pc -= imm;
                    }
                },
                _ => todo!(),
            }
        }
        Ok(())
    }

    pub fn dump_registers(&self) {
        println!("r0: {} pc: {}", self.r0, self.pc);
    }

    pub fn reset(&mut self) {
        self.r0 = 0;
        self.pc = 0;
        self.ran_pcs = Vec::default();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_instruction_decode() {
        assert_eq!(Instruction::Nop, Instruction::decode("nop"));
        assert_eq!(Instruction::Nop, Instruction::decode("nop +34"));

        assert_eq!(
            Instruction::AccImmediate(ArithmeticOp::PLUS, 3),
            Instruction::decode("acc +3")
        );
        assert_eq!(
            Instruction::AccImmediate(ArithmeticOp::MINUS, 5),
            Instruction::decode("acc -5")
        );
        assert_eq!(
            Instruction::AccImmediate(ArithmeticOp::MINUS, 55),
            Instruction::decode("acc -55")
        );

        assert_eq!(
            Instruction::JmpRelative(ArithmeticOp::PLUS, 3),
            Instruction::decode("jmp +3")
        );
        assert_eq!(
            Instruction::JmpRelative(ArithmeticOp::MINUS, 5),
            Instruction::decode("jmp -5")
        );
        assert_eq!(
            Instruction::JmpRelative(ArithmeticOp::MINUS, 55),
            Instruction::decode("jmp -55")
        );
    }

    #[test]
    fn test_run_code() {
        let mut handheld = HandHeld::default();
        handheld.run_code(&["jmp +2", "acc +5", "acc +10"]).unwrap();

        assert_eq!(10, handheld.r0);
        assert_eq!(3, handheld.pc);
    }

    #[test]
    fn test_run_code_loop_detect() {
        let mut handheld = HandHeld::default();
        let ret = handheld.run_code(&["jmp +2", "acc +5", "jmp -2"]);

        assert_eq!(CpuFault::Loop(0), ret.unwrap_err());
        assert_eq!(0, handheld.r0);
    }

    #[test]
    fn test_reset() {
        let mut handheld = HandHeld::default();
        handheld.run_code(&["jmp +2", "acc +5", "acc +10"]).unwrap();

        assert_eq!(10, handheld.r0);
        assert_eq!(3, handheld.pc);

        handheld.reset();

        assert_eq!(0, handheld.r0);
        assert_eq!(0, handheld.pc);
    }
}
