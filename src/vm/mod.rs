use crate::control_rom::{AddrMux, AluMux, DataMux, DestMux, ImmediateMux, MemoryAccess};
use crate::io_devices::{Device, IoDevices};
use crate::registers::call_stack::CallStack;
use crate::registers::data_memory::MemoryState;
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
    pub data_memory: DataMemory,
    pub io_devices: IoDevices,
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
        let io_devices = IoDevices::default();
        VM {
            alu,
            reg_file,
            control_rom,
            instruction_memory,
            pc,
            call_stack,
            data_memory,
            io_devices,
        }
    }

    pub fn execute_program(&mut self, file_path: impl AsRef<Path>) -> crate::Result<()> {
        let file_path = file_path.as_ref();
        self.load_program(file_path)?;
        while self.clock() != OPCODE_HLT {}
        Ok(())
    }

    pub fn load_program(&mut self, file_path: impl AsRef<Path>) -> crate::Result<()> {
        self.pc.value = Bits::from(0u16).resize();
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

    fn process_instruction(&mut self, instruction: ProgramInstruction) {
        let opcode = instruction.slice(12);
        let control_signals = self.control_rom.get_control_signals(opcode);

        self.call_stack.state = control_signals.call_stack_state;
        self.reg_file.enable(control_signals.reg_file_enable);
        self.alu.set_setting(control_signals.alu_settings);
        match control_signals.memory_access {
            MemoryAccess::Read => {
                self.data_memory.enabled = true;
                self.data_memory.set_state(MemoryState::Read);
            }
            MemoryAccess::Write => {
                self.data_memory.enabled = true;
                self.data_memory.set_state(MemoryState::Write);
            }
            MemoryAccess::Disabled => {
                self.data_memory.set_state(MemoryState::Disabled);
                self.data_memory.enabled = false;
            }
        }
        self.alu.set_flags = control_signals.set_flags;

        let current_pc = self.pc.value;
        let pc_inc = current_pc + Bits::from(1u16).resize::<10>();

        self.call_stack.push(pc_inc);
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

        let r1 = instruction.slice(8);
        let r2 = instruction.slice(4);
        self.reg_file.set_read_addresses([r1, r2]);

        let [a, b] = self.reg_file.read_outputs;

        let immediate = match control_signals.immediate_mux {
            ImmediateMux::Immediate => instruction.slice(0),
            ImmediateMux::Offset => instruction.slice::<4>(0).resize::<8>(),
        };

        let alu_input_b = match control_signals.alu_mux {
            AluMux::R2 => b,
            AluMux::BypassRegisterFile => immediate,
        };
        let alu_result = self.alu.compute(a, alu_input_b);

        let data = match control_signals.data_mux {
            DataMux::Alu => alu_result,
            DataMux::Immediate => instruction.slice(0),
            DataMux::Memory => {
                if alu_result >= Bits::from(240u8) {
                    // If the ALU result is an I/O address, read from the corresponding device
                    self.io_devices.on_read(alu_result)
                } else {
                    // Otherwise, read from the data memory
                    self.data_memory.read(alu_result)
                }
            }
        };
        if alu_result >= Bits::from(240u8) {
            // If the ALU result is an I/O address, write to the corresponding device
            self.io_devices.on_write(alu_result, b);
        } else {
            // Otherwise, write to the data memory
            self.data_memory.schedule_write((alu_result, b));
        }

        let write_address = match control_signals.dest_mux {
            DestMux::First => instruction.slice(8),
            DestMux::Second => instruction.slice(4),
            DestMux::Third => instruction.slice(0),
        };

        self.reg_file.schedule_write((write_address, data));

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
        self.data_memory.clock();
        instruction.slice(12)
    }
}
