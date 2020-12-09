#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ArithmeticOp {
    PLUS,
    MINUS,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Instruction {
    Nop,
    AccImmediate(ArithmeticOp, usize),
    JmpRelative(ArithmeticOp, usize),
}

impl Instruction {
    // TODO: do not panic
    pub fn decode(in_string: &str) -> Self {
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
}
