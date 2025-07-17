use crate::bits::Bits;

type ProgramInstruction = [Bits<4>; 4];
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct InstructionMemory {
    pub(crate) instructions: [[Bits<4>; 4]; 1024],
}

impl Default for InstructionMemory {
    fn default() -> Self {
        InstructionMemory {
            instructions: [[Bits::from(0u8).resize(); 4]; 1024],
        }
    }
}

impl InstructionMemory {
    pub(crate) fn load_instructions(&mut self, instructions: Vec<ProgramInstruction>) {
        for (i, instruction) in instructions.into_iter().enumerate() {
            if i < self.instructions.len() {
                self.instructions[i] = instruction;
            } else {
                panic!("Instruction memory overflow");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_instruction(val: u8) -> ProgramInstruction {
        [
            Bits::from(val).resize(),
            Bits::from(val).resize(),
            Bits::from(val).resize(),
            Bits::from(val).resize(),
        ]
    }

    #[test]
    #[should_panic(expected = "Instruction memory overflow")]
    fn test_load_instructions_overflow_panics() {
        let mut mem = InstructionMemory::default();
        let instructions = vec![make_instruction(0); 1025];
        mem.load_instructions(instructions);
    }
}
