
#[allow(unused_imports)]
use super::*;
#[test]
fn add() {
    let set = ControlRom.get_control_signals(Bits::from(2u8).resize());
    assert_eq!(
        set,
        ControlSignals {
            alu_settings: AluSettings::Add,
            reg_files_enable: true,
            set_flags: true,
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
            reg_files_enable: true,
            set_flags: true,
            ..Default::default()
        }
    );
}

#[test]
fn nor() {
    let set = ControlRom.get_control_signals(Bits::from(4u8).resize());
    assert_eq!(
        set,
        ControlSignals {
            alu_settings: AluSettings::Nor,
            reg_files_enable: true,
            set_flags: true,
            ..Default::default()
        }
    );
}

#[test]
fn and() {
    let set = ControlRom.get_control_signals(Bits::from(5u8).resize());
    assert_eq!(
        set,
        ControlSignals {
            alu_settings: AluSettings::And,
            reg_files_enable: true,
            set_flags: true,
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
            reg_files_enable: true,
            set_flags: true,
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
            reg_files_enable: true,
            set_flags: true,
            ..Default::default()
        }
    );
}
