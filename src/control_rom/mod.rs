use crate::{alu::AluSettings, bits::Bits};

#[derive(Debug, Default)]
pub(crate) struct ControlRom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum AddrMux {
    #[default]
    Increment,
    Jump,
    Return,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum DataMux {
    #[default]
    Alu,
    Immediate,
    Memory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum DestMux {
    First,
    Second,
    #[default]
    Third,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum MemoryAccess {
    #[default]
    Disabled,
    Read,
    Write,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum AluMux {
    #[default]
    R2,
    BypassRegisterFile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum CallStackState {
    #[default]
    Disabled,
    Push,
    Pop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum ImmediateMux {
    #[default]
    Immediate,
    Offset,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct ControlSignals {
    pub(crate) alu_settings: AluSettings,
    pub(crate) reg_file_enable: bool,
    pub(crate) data_mux: DataMux,
    pub(crate) dest_mux: DestMux,
    pub(crate) alu_mux: AluMux,
    pub(crate) addr_mux: AddrMux,
    pub(crate) is_branch: bool,
    pub(crate) set_flags: bool,
    pub(crate) call_stack_state: CallStackState,
    pub(crate) immediate_mux: ImmediateMux,
    pub(crate) memory_access: MemoryAccess,
}

impl ControlRom {
    pub(crate) fn get_control_signals(&self, opcode: Bits<4>) -> ControlSignals {
        match opcode.to_string().as_str() {
            // NOP
            "0000" => ControlSignals {
                ..Default::default()
            },
            // HLT
            "0001" => ControlSignals {
                ..Default::default()
            },
            // ADD
            "0010" => ControlSignals {
                alu_settings: AluSettings::Add,
                reg_file_enable: true,
                set_flags: true,
                ..Default::default()
            },
            // SUB
            "0011" => ControlSignals {
                alu_settings: AluSettings::Sub,
                reg_file_enable: true,
                set_flags: true,
                ..Default::default()
            },
            // NOR
            "0100" => ControlSignals {
                alu_settings: AluSettings::Nor,
                reg_file_enable: true,
                set_flags: true,
                ..Default::default()
            },
            // AND
            "0101" => ControlSignals {
                alu_settings: AluSettings::And,
                reg_file_enable: true,
                set_flags: true,
                ..Default::default()
            },
            // XOR
            "0110" => ControlSignals {
                alu_settings: AluSettings::Xor,
                reg_file_enable: true,
                set_flags: true,
                ..Default::default()
            },
            // RSH
            "0111" => ControlSignals {
                alu_settings: AluSettings::Rshift,
                reg_file_enable: true,
                set_flags: true, // Change from original design, RSH should set flags
                ..Default::default()
            },
            // LDI
            "1000" => ControlSignals {
                reg_file_enable: true,
                data_mux: DataMux::Immediate,
                dest_mux: DestMux::First,
                immediate_mux: ImmediateMux::Immediate,
                ..Default::default()
            },
            // ADI
            "1001" => ControlSignals {
                alu_settings: AluSettings::Add,
                reg_file_enable: true,
                dest_mux: DestMux::First,
                alu_mux: AluMux::BypassRegisterFile,
                immediate_mux: ImmediateMux::Immediate,
                data_mux: DataMux::Alu,
                set_flags: true,
                ..Default::default()
            },
            // JMP
            "1010" => ControlSignals {
                addr_mux: AddrMux::Jump,
                ..Default::default()
            },
            // BRH
            "1011" => ControlSignals {
                is_branch: true,
                ..Default::default()
            },
            // CAL
            "1100" => ControlSignals {
                call_stack_state: CallStackState::Push,
                addr_mux: AddrMux::Jump,
                ..Default::default()
            },
            // RET
            "1101" => ControlSignals {
                addr_mux: AddrMux::Return,
                call_stack_state: CallStackState::Pop,
                ..Default::default()
            },
            // LOD, TODO: implement
            "1110" => ControlSignals {
                immediate_mux: ImmediateMux::Offset,
                alu_settings: AluSettings::Add,
                reg_file_enable: true,
                data_mux: DataMux::Memory,
                dest_mux: DestMux::Second,
                alu_mux: AluMux::BypassRegisterFile,
                memory_access: MemoryAccess::Read,
                ..Default::default()
            },
            // STR, TODO: implement
            "1111" => ControlSignals {
                alu_settings: AluSettings::Add,
                reg_file_enable: true,
                alu_mux: AluMux::BypassRegisterFile,
                immediate_mux: ImmediateMux::Offset,
                memory_access: MemoryAccess::Write,
                ..Default::default()
            },
            #[allow(clippy::panic)]
            _ => panic!("Not yet implemented"), //eventually unreachable
        }
    }
}

#[cfg(test)]
mod tests;
