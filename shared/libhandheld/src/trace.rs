use crate::instruction::Instruction;

#[derive(Debug, PartialEq)]
pub struct TraceEntry {
    clock: u128,
    r0: i64,
    pc: usize,
    instruction: Instruction,
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
    pub fn serialize(&self) -> String {
        format!(
            "{} {} {} {:?}",
            self.clock, self.r0, self.pc, self.instruction
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

        assert_eq!(
            "0 0 1 AccImmediate(PLUS, 3)\n0 0 1 AccImmediate(PLUS, 3)",
            tracer.serialize()
        );
    }
}
