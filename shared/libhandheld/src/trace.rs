use crate::instruction::Instruction;

#[derive(Debug, PartialEq)]
pub struct TraceEntry {
    pub clock: u128,
    pub r0: i64,
    pub pc: usize,
    pub instruction: Instruction,
}

impl TraceEntry {
    pub fn new(clock: u128, r0: i64, pc: usize, instruction: Instruction) -> Self {
        Self {
            clock,
            r0,
            pc,
            instruction,
        }
    }

    pub fn parse(in_str: &str) -> Option<Self> {
        let mut split = in_str.split(" ");
        let clock: u128 = split.next()?.parse().ok()?;
        let r0: i64 = split.next()?.parse().ok()?;
        let pc: usize = split.next()?.parse().ok()?;
        let instruction = split.collect::<Vec<_>>().join(" ");
        Some(Self::new(clock, r0, pc, Instruction::decode(&instruction)))
    }

    pub fn serialize(&self) -> String {
        format!(
            "{} {} {} {}",
            self.clock,
            self.r0,
            self.pc,
            self.instruction.encode()
        )
    }
}

// TODO: Threading for async tracing
/// A very, very naive and propably quite slow tracing implementation
#[derive(Debug)]
pub struct Tracer {
    entries: Vec<TraceEntry>,
}

impl Tracer {
    pub fn new() -> Self {
        Self {
            entries: Vec::with_capacity(1024),
        }
    }

    pub fn log(&mut self, entry: TraceEntry) {
        self.entries.push(entry);
    }

    pub fn serialize(&self) -> String {
        self.entries
            .iter()
            .map(|e| e.serialize())
            .fold(String::new(), |mut acc, e| {
                if acc != "" {
                    acc.push_str("\n");
                }
                acc.push_str(e.as_str());
                acc
            })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instruction::ArithmeticOp;
    #[test]
    fn test_tracing() {
        let mut tracer = Tracer::new();
        tracer.log(TraceEntry::new(
            0u128,
            0i64,
            1usize,
            Instruction::decode("acc +3"),
        ));
        tracer.log(TraceEntry::new(
            0u128,
            0i64,
            1usize,
            Instruction::decode("acc +3"),
        ));

        assert_eq!("0 0 1 acc +3\n0 0 1 acc +3", tracer.serialize());
    }

    #[test]
    fn test_parse() {
        let expected = TraceEntry::new(0, 0, 0, Instruction::AccImmediate(ArithmeticOp::Plus, 5));
        assert_eq!(Some(expected), TraceEntry::parse("0 0 0 acc +5"));

        let expected = TraceEntry::new(1, 2, 3, Instruction::JmpRelative(ArithmeticOp::Minus, 4));
        assert_eq!(Some(expected), TraceEntry::parse("1 2 3 jmp -4"));
    }
}
