use super::super::*;
#[test]
fn test_disable_resets_read_values() {
    let mut reg_file = RegisterFile::default();
    reg_file.enable();

    reg_file.schedule_write(2, Bits::from(4u8));
    reg_file.clock();
    reg_file.set_read_addresses([2, 2]);

    reg_file.disable();

    assert_eq!(&reg_file.read_outputs[0], &Bits::from(0u8));
    assert_eq!(&reg_file.read_outputs[1], &Bits::from(0u8));
}

#[test]
fn test_enable_restores_read_values() {
    let mut reg_file = RegisterFile::default();
    reg_file.enable();

    reg_file.schedule_write(2, Bits::from(4u8));
    reg_file.clock();
    reg_file.set_read_addresses([2, 2]);
    reg_file.disable();

    reg_file.enable();

    assert_eq!(&reg_file.read_outputs[0], &Bits::from(4u8));
    assert_eq!(&reg_file.read_outputs[1], &Bits::from(4u8));
}
