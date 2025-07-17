#![allow(dead_code)]
use std::{path::Path, str::FromStr};

use crate::{
    alu::Alu, bits::Bits, control_rom::ControlRom, instruction_memory::InstructionMemory,
    program_counter::PC, register::RegisterFile,
};
mod alu;
mod bits;
mod control_rom;
mod instruction_memory;
mod parser;
mod program_counter;
mod register;

pub type ProgramInstruction = [Bits<4>; 4];
pub type Program = Vec<ProgramInstruction>;
type OpCode = Bits<4>;
const OPCODE_HLT: Bits<4> = Bits {
    bit_array: [true, false, false, false],
};

#[derive(Debug)]
pub struct VM {
    alu: Alu,
    pub reg_file: RegisterFile,
    control_rom: ControlRom,
    instruction_memory: InstructionMemory,
    pc: PC,
}

impl VM {
    pub fn new() -> Self {
        let alu = Alu::default();
        let reg_file = RegisterFile::default();
        let control_rom = ControlRom;
        let instruction_memory = InstructionMemory::default();
        let pc = PC::default();
        VM {
            alu,
            reg_file,
            control_rom,
            instruction_memory,
            pc,
        }
    }

    pub fn execute_program(&mut self, file_path: impl AsRef<Path>) {
        let file_path = file_path.as_ref();
        self.load_program(file_path);
        while self.clock() != OPCODE_HLT {}
    }

    pub fn load_program(&mut self, file_path: impl AsRef<Path>) {
        let file_path = file_path.as_ref();
        parser::parse_program(file_path).expect("Failed to parse file");
        let path = Path::new(file_path).with_extension("mc");
        let program = std::fs::read_to_string(path).expect("Failed to read file");

        let content_bits: Vec<[Bits<4>; 4]> = program
            .lines()
            .map(|line| {
                let bits_vec: Vec<Bits<4>> = line
                    .split_whitespace()
                    .map(|s| Bits::from_str(s).unwrap())
                    .collect();
                bits_vec
                    .try_into()
                    .expect("Each instruction must have exactly 4 fields")
            })
            .collect();

        self.instruction_memory.load_instructions(content_bits);
    }

    pub fn process_instruction(&mut self, instruction: ProgramInstruction) {
        let opcode = instruction[0];
        let control_signals = self.control_rom.get_control_signals(opcode);
        self.alu.set_setting(control_signals.alu_settings);
        if control_signals.enable {
            self.reg_file.enable();
        } else {
            self.reg_file.disable();
        }

        let r1 = instruction[1];
        let r2 = instruction[2];
        let mut write_adress = instruction[3];
        self.reg_file.set_read_addresses([r1, r2]);
        let mut data = self
            .alu
            .compute(self.reg_file.read_outputs[0], self.reg_file.read_outputs[1]);
        if control_signals.data_mux {
            // Combine two Bits<4> into a Bits<8> by concatenating their bit arrays
            let mut combined = [false; 8];
            combined[4..].copy_from_slice(&r2.bit_array);
            combined[..4].copy_from_slice(&write_adress.bit_array);
            data = Bits::<8>::from(combined);
        }
        if control_signals.dest_mux {
            write_adress = r1;
        }
        self.reg_file.schedule_write(write_adress, data);
    }

    fn clock(&mut self) -> OpCode {
        // TODO: implement sliceindex
        let instr_adr = self.pc.clock().to_usize();
        // TODO: handle out of bounds instruction address
        if instr_adr >= self.instruction_memory.instructions.len() {
            return OPCODE_HLT;
        }
        let instruction = self.instruction_memory.instructions[instr_adr];
        self.process_instruction(instruction);
        self.reg_file.clock();
        instruction[0]
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: make this an integration test
#[cfg(test)]
mod tests {
    use crate::register::RegisterBank;
    use crate::{Bits, RegisterFile, VM};

    #[test]
    fn test_vm_execute_program() {
        let mut vm = VM::default();
        vm.execute_program("test.as");
        vm.reg_file.display(); // Display the register state after execution, should be all zero as the registers are initialized to zero
    }

    #[test]
    fn test_vm_execution() {
        let mut arr = [Bits::from(0u8); 16];
        arr[1] = Bits::from(7u8);
        arr[2] = Bits::from(8u8);
        arr[3] = Bits::from(6u8);
        let mut vm = VM::new();
        vm.reg_file = RegisterFile::new(RegisterBank::from(arr));
        vm.execute_program("test.as");
        assert_eq!(vm.reg_file.register_banks[0][1].to_usize(), 7,);
        assert_eq!(vm.reg_file.register_banks[0][2].to_usize(), 7,);
        assert_eq!(vm.reg_file.register_banks[0][3].to_usize(), 9,);
        assert_eq!(vm.reg_file.register_banks[1][1].to_usize(), 7,);
        assert_eq!(vm.reg_file.register_banks[1][2].to_usize(), 7,);
        assert_eq!(vm.reg_file.register_banks[1][3].to_usize(), 9,);
    }

    #[test]
    fn test_nop() {
        std::fs::write("nop.as", "NOP\n").unwrap();
        std::fs::write("nop.as", "HLT").unwrap();
        let mut vm = VM::default();
        vm.execute_program("nop.as");
        std::fs::remove_file("nop.as").unwrap();
        std::fs::remove_file("nop.mc").unwrap();
    }

    #[test]
    fn test_vm_execute_program_2() {
        let mut arr = [Bits::from(0u8); 16];
        arr[1] = Bits::from(1u8);
        arr[2] = Bits::from(1u8);
        let mut vm = VM::new();
        vm.reg_file = RegisterFile::new(RegisterBank::from(arr));
        vm.execute_program("test2.as");
        assert_eq!(vm.reg_file.register_banks[0][1].to_usize(), 3);
        assert_eq!(vm.reg_file.register_banks[0][2].to_usize(), 1);
        assert_eq!(vm.reg_file.register_banks[0][3].to_usize(), 2);
        assert_eq!(vm.reg_file.register_banks[0][4].to_usize(), 3);

        assert_eq!(vm.reg_file.register_banks[1][1].to_usize(), 3);
        assert_eq!(vm.reg_file.register_banks[1][2].to_usize(), 1);
        assert_eq!(vm.reg_file.register_banks[1][3].to_usize(), 2);
        assert_eq!(vm.reg_file.register_banks[1][4].to_usize(), 3);
    }

    #[test]
    fn test_vm_program_3() {
        let mut vm = VM::default();
        vm.execute_program("test3.as");
        assert_eq!(vm.reg_file.register_banks[0][1].to_usize(), 4);
        assert_eq!(vm.reg_file.register_banks[0][2].to_usize(), 8);
    }

    #[test]
    fn test_fib() {
        let mut vm = VM::default();
        vm.execute_program("fib.as");
        assert_eq!(vm.reg_file.register_banks[0][1].to_usize(), 1);
        assert_eq!(vm.reg_file.register_banks[0][2].to_usize(), 1);
        assert_eq!(vm.reg_file.register_banks[0][3].to_usize(), 2);
        assert_eq!(vm.reg_file.register_banks[0][4].to_usize(), 3);
        assert_eq!(vm.reg_file.register_banks[0][5].to_usize(), 5);
    }
}
