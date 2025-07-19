use crate::control_rom::AddrMux;
use crate::registers::call_stack::CallStack;
use crate::registers::Register;
use crate::{
    alu::Alu, bits::Bits, control_rom::ControlRom, instruction_memory::InstructionMemory,
    parser::parse_as_instruction, program_counter::PC, registers::data_memory::DataMemory,
    registers::RegisterFile, OpCode, ProgramInstruction,
};
use std::path::Path;

const OPCODE_HLT: Bits<4> = Bits {
    bit_array: [true, false, false, false],
};

#[derive(Debug, Default)]
pub struct VM {
    alu: Alu,
    pub reg_file: RegisterFile,
    control_rom: ControlRom,
    instruction_memory: InstructionMemory,
    pc: PC,
    call_stack: CallStack,
    data_memory: DataMemory,
}

impl VM {
    pub fn new() -> Self {
        let alu = Alu::default();
        let reg_file = RegisterFile::default();
        let control_rom = ControlRom;
        let instruction_memory = InstructionMemory::default();
        let pc = PC::default();
        let call_stack = CallStack::new();
        let data_memory = DataMemory::default();
        VM {
            alu,
            reg_file,
            control_rom,
            instruction_memory,
            pc,
            call_stack,
            data_memory,
        }
    }

    pub fn execute_program(&mut self, file_path: impl AsRef<Path>) -> crate::Result<()> {
        let file_path = file_path.as_ref();
        self.load_program(file_path)?;
        while self.clock() != OPCODE_HLT {}
        Ok(())
    }

    pub fn load_program(&mut self, file_path: impl AsRef<Path>) -> crate::Result<()> {
        let file_path = file_path.as_ref();
        crate::parser::parse_program(file_path)?;
        let path = Path::new(file_path).with_extension("mc");
        let program = std::fs::read_to_string(path)?;
        let content_bits = program
            .lines()
            .map(parse_as_instruction)
            .collect::<Vec<_>>();
        self.instruction_memory.load_instructions(content_bits)?;
        Ok(())
    }

    // TODO: process STR/LOD
    pub fn process_instruction(&mut self, instruction: ProgramInstruction) {
        let opcode = instruction.slice(12);
        let control_signals = self.control_rom.get_control_signals(opcode);

        let current_pc = self.pc.value;
        let pc_inc = current_pc + Bits::from(1u16).resize::<10>();
        let mut next_pc = match control_signals.addr_mux {
            AddrMux::Increment => pc_inc,
            AddrMux::Jump => instruction.slice(0),
            AddrMux::Return => {
                match self.call_stack.pop() {
                    Some(addr) => addr,
                    None => {
                        // TODO add proper error handling
                        eprintln!("Call stack underflow, returning to next instruction");
                        pc_inc
                    }
                }
            }
        };
        if control_signals.is_branch {
            let condition = instruction.slice(10);
            if self.alu.flags.cond_true(condition) {
                next_pc = instruction.slice(0);
            }
        }
        if control_signals.is_call {
            self.call_stack
                .push(current_pc + Bits::from(1u16).resize::<10>());
            next_pc = instruction.slice(0);
        }

        self.alu.set_setting(control_signals.alu_settings);
        self.reg_file.enable(control_signals.reg_files_enable);
        let r1 = instruction.slice(8);
        let r2 = instruction.slice(4);
        let mut write_adress = instruction.slice(0);
        self.reg_file.set_read_addresses([r1, r2]);
        let [a, mut b] = self.reg_file.read_outputs;
        if control_signals.alu_mux {
            b = instruction.slice(0);
        }
        let data = if control_signals.set_flags {
            self.alu.compute(a, b)
        } else if control_signals.data_mux {
            instruction.slice(0)
        } else {
            // TODO: properly handle
            Bits::from(0u8)
        };
        if control_signals.dest_mux {
            write_adress = r1;
        }
        self.reg_file.schedule_write((write_adress, data));
        self.pc.value = next_pc;
    }

    fn clock(&mut self) -> OpCode {
        let instr_adr = self.pc.clock().to_usize();
        if instr_adr >= self.instruction_memory.instructions.len() {
            return OPCODE_HLT;
        }
        let instruction = self.instruction_memory.instructions[instr_adr];
        self.process_instruction(instruction);
        self.reg_file.clock();
        self.call_stack.stack.clock();
        instruction.slice(12)
    }
}

#[cfg(test)]
mod tests;
