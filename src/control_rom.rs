use crate::{alu::AluSettings, bits::Bits};

#[derive(Debug, Default)]
pub(crate) struct ControlRom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct ControlSignals {
    pub(crate) alu_settings: AluSettings,
    pub(crate) enable: bool,
    pub(crate) data_mux: bool,
    pub(crate) dest_mux: bool,
    pub(crate) alu_mux: bool,
}

impl ControlRom {
    pub(crate) fn get_control_signals(&self, opcode: Bits<4>) -> ControlSignals {
        match opcode.to_string().as_str() {
            "0000" => ControlSignals {
                enable: false,
                ..Default::default()
            },
            "0001" => ControlSignals {
                enable: false,
                ..Default::default()
            },
            "0010" => ControlSignals {
                alu_settings: AluSettings::Add,
                enable: true,
                ..Default::default()
            },
            "0011" => ControlSignals {
                alu_settings: AluSettings::Sub,
                enable: true,
                ..Default::default()
            },
            "0100" => ControlSignals {
                alu_settings: AluSettings::And,
                enable: true,
                ..Default::default()
            },
            "0101" => ControlSignals {
                alu_settings: AluSettings::Nor,
                enable: true,
                ..Default::default()
            },
            "0110" => ControlSignals {
                alu_settings: AluSettings::Xor,
                enable: true,
                ..Default::default()
            },
            "0111" => ControlSignals {
                alu_settings: AluSettings::Rshift,
                enable: true,
                ..Default::default()
            },
            "1000" => ControlSignals {
                enable: true,
                data_mux: true,
                dest_mux: true,
                ..Default::default()
            },
            "1001" => ControlSignals {
                alu_settings: AluSettings::Add,
                enable: true,
                dest_mux: true,
                alu_mux: true,
                ..Default::default()
            },
            #[allow(clippy::panic)]
            _ => panic!("Not yet implemented"), //eventually unreachable
        }
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn add() {
        let set = ControlRom.get_control_signals(Bits::from(2u8).resize());
        assert_eq!(
            set,
            ControlSignals {
                alu_settings: AluSettings::Add,
                enable: true,
                ..Default::default()
            }
        );
    }

    #[test]
    fn sub() {
        let set = ControlRom.get_control_signals(Bits::from(3u8).resize());
        assert_eq!(
            set,
            ControlSignals {
                alu_settings: AluSettings::Sub,
                enable: true,
                ..Default::default()
            }
        );
    }

    #[test]
    fn and() {
        let set = ControlRom.get_control_signals(Bits::from(4u8).resize());
        assert_eq!(
            set,
            ControlSignals {
                alu_settings: AluSettings::And,
                enable: true,
                ..Default::default()
            }
        );
    }

    #[test]
    fn nor() {
        let set = ControlRom.get_control_signals(Bits::from(5u8).resize());
        assert_eq!(
            set,
            ControlSignals {
                alu_settings: AluSettings::Nor,
                enable: true,
                ..Default::default()
            }
        );
    }

    #[test]
    fn xor() {
        let set = ControlRom.get_control_signals(Bits::from(6u8).resize());
        assert_eq!(
            set,
            ControlSignals {
                alu_settings: AluSettings::Xor,
                enable: true,
                ..Default::default()
            }
        );
    }

    #[test]
    fn rshift() {
        let set = ControlRom.get_control_signals(Bits::from(7u8).resize());
        assert_eq!(
            set,
            ControlSignals {
                alu_settings: AluSettings::Rshift,
                enable: true,
                ..Default::default()
            }
        );
    }

    #[test]
    #[should_panic]
    fn not_yet_implemented() {
        let _ = ControlRom.get_control_signals(Bits::from(15u8).resize());
    }
}
