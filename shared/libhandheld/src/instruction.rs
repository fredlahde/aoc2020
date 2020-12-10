#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ArithmeticOp {
    Plus,
    Minus,
}

impl ArithmeticOp {
    fn decode(in_char: &char) -> Self {
        match in_char {
            '+' => ArithmeticOp::Plus,
            '-' => ArithmeticOp::Minus,
            _ => panic!("invalid ArithmeticOp"),
        }
    }

    fn encode(&self) -> String {
        match self {
            ArithmeticOp::Plus => "+",
            ArithmeticOp::Minus => "-",
        }
        .to_owned()
    }
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
        let op_arg = ArithmeticOp::decode(&(*op_arg as char));
        let op_imm = &op[1..];
        let op_imm = std::str::from_utf8(op_imm).unwrap();
        let op_imm: usize = op_imm.parse().unwrap();

        match inst {
            "acc" => Instruction::AccImmediate(op_arg, op_imm),
            "jmp" => Instruction::JmpRelative(op_arg, op_imm),
            _ => panic!("SIGILL"),
        }
    }

    pub fn encode(&self) -> String {
        match self {
            Instruction::Nop => format!("nop"),
            Instruction::AccImmediate(op, imm) => format!("{} {}{}", "acc", op.encode(), imm),
            Instruction::JmpRelative(op, imm) => format!("{} {}{}", "jmp", op.encode(), imm),
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
            Instruction::AccImmediate(ArithmeticOp::Plus, 3),
            Instruction::decode("acc +3")
        );
        assert_eq!(
            Instruction::AccImmediate(ArithmeticOp::Minus, 5),
            Instruction::decode("acc -5")
        );
        assert_eq!(
            Instruction::AccImmediate(ArithmeticOp::Minus, 55),
            Instruction::decode("acc -55")
        );

        assert_eq!(
            Instruction::JmpRelative(ArithmeticOp::Plus, 3),
            Instruction::decode("jmp +3")
        );
        assert_eq!(
            Instruction::JmpRelative(ArithmeticOp::Minus, 5),
            Instruction::decode("jmp -5")
        );
        assert_eq!(
            Instruction::JmpRelative(ArithmeticOp::Minus, 55),
            Instruction::decode("jmp -55")
        );
    }

    #[test]
    fn test_instruction_encode() {
        assert_eq!("nop", Instruction::Nop.encode());

        assert_eq!(
            "acc +5",
            Instruction::AccImmediate(ArithmeticOp::Plus, 5).encode()
        );
        assert_eq!(
            "acc -15",
            Instruction::AccImmediate(ArithmeticOp::Minus, 15).encode()
        );
        assert_eq!(
            "jmp +5",
            Instruction::JmpRelative(ArithmeticOp::Plus, 5).encode()
        );
        assert_eq!(
            "jmp -15",
            Instruction::JmpRelative(ArithmeticOp::Minus, 15).encode()
        );
    }

    #[test]
    fn test_op_decode() {
        assert_eq!(ArithmeticOp::Plus, ArithmeticOp::decode(&'+'));
        assert_eq!(ArithmeticOp::Minus, ArithmeticOp::decode(&'-'));
    }

    #[test]
    fn test_op_encode() {
        assert_eq!("+", ArithmeticOp::Plus.encode());
        assert_eq!("-", ArithmeticOp::Minus.encode());
    }
}
