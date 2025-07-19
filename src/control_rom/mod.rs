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
pub(crate) struct ControlSignals {
    pub(crate) alu_settings: AluSettings,
    pub(crate) reg_files_enable: bool,
    pub(crate) data_mux: bool,
    pub(crate) dest_mux: bool,
    pub(crate) alu_mux: bool,
    pub(crate) addr_mux: AddrMux,
    pub(crate) is_branch: bool,
    pub(crate) set_flags: bool,
    pub(crate) is_call: bool,
}

impl ControlRom {
    pub(crate) fn get_control_signals(&self, opcode: Bits<4>) -> ControlSignals {
        match opcode.to_string().as_str() {
            // NOP
            "0000" => ControlSignals {
                reg_files_enable: false,
                set_flags: false,
                ..Default::default()
            },
            // HLT
            "0001" => ControlSignals {
                reg_files_enable: false,
                set_flags: false,
                ..Default::default()
            },
            // ADD
            "0010" => ControlSignals {
                alu_settings: AluSettings::Add,
                reg_files_enable: true,
                set_flags: true,
                ..Default::default()
            },
            // SUB
            "0011" => ControlSignals {
                alu_settings: AluSettings::Sub,
                reg_files_enable: true,
                set_flags: true,
                ..Default::default()
            },
            // NOR
            "0100" => ControlSignals {
                alu_settings: AluSettings::Nor,
                reg_files_enable: true,
                set_flags: true,
                ..Default::default()
            },
            // AND
            "0101" => ControlSignals {
                alu_settings: AluSettings::And,
                reg_files_enable: true,
                set_flags: true,
                ..Default::default()
            },
            // XOR
            "0110" => ControlSignals {
                alu_settings: AluSettings::Xor,
                reg_files_enable: true,
                set_flags: true,
                ..Default::default()
            },
            // RSH
            "0111" => ControlSignals {
                alu_settings: AluSettings::Rshift,
                reg_files_enable: true,
                set_flags: true,
                ..Default::default()
            },
            // LDI
            "1000" => ControlSignals {
                reg_files_enable: true,
                data_mux: true,
                dest_mux: true,
                set_flags: false,
                ..Default::default()
            },
            // ADI
            "1001" => ControlSignals {
                alu_settings: AluSettings::Add,
                reg_files_enable: true,
                dest_mux: true,
                alu_mux: true,
                set_flags: true,
                ..Default::default()
            },
            // JMP
            "1010" => ControlSignals {
                addr_mux: AddrMux::Jump,
                set_flags: false,
                ..Default::default()
            },
            // BRH
            "1011" => ControlSignals {
                is_branch: true,
                set_flags: false,
                ..Default::default()
            },
            // CAL
            "1100" => ControlSignals {
                is_call: true,
                ..Default::default()
            },
            // RET
            "1101" => ControlSignals {
                addr_mux: AddrMux::Return,
                ..Default::default()
            },
            // LOD, TODO: implement
            "1110" => ControlSignals {
                ..Default::default()
            },
            // STR, TODO: implement
            "1111" => ControlSignals {
                ..Default::default()
            },
            #[allow(clippy::panic)]
            _ => panic!("Not yet implemented"), //eventually unreachable
        }
    }
}

#[cfg(test)]
mod tests;
