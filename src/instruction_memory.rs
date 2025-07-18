use crate::{bits::Bits, ProgramInstruction};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct InstructionMemory {
    pub(crate) instructions: [ProgramInstruction; 1024],
}

impl Default for InstructionMemory {
    fn default() -> Self {
        Self {
            instructions: [Bits::from(0u16); 1024],
        }
    }
}

impl InstructionMemory {
    pub(crate) fn load_instructions(
        &mut self,
        instructions: Vec<ProgramInstruction>,
    ) -> crate::Result<()> {
        for (i, instruction) in instructions.into_iter().enumerate() {
            if i < self.instructions.len() {
                self.instructions[i] = instruction;
            } else {
                return Err(crate::Error::InstructionMemoryOverflow);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_instruction(val: u16) -> ProgramInstruction {
        Bits::from(val)
    }

    #[allow(clippy::panic)]
    #[test]
    fn load_instructions_overflow_panics() {
        let mut mem = InstructionMemory::default();
        let instructions = vec![make_instruction(0); 1025];
        let err = mem.load_instructions(instructions);
        match err {
            Err(crate::Error::InstructionMemoryOverflow) => {}
            _ => panic!("Expected InstructionMemoryOverflow error"),
        }
    }
}
