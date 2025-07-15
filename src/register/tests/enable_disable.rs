use super::super::*;
#[test]
fn test_disable_resets_read_values() {
    let mut reg_file = RegisterFile::default();
    reg_file.enable();

    reg_file.schedule_write(Bits::from(2u8).resize(), Bits::from(4u8).resize());
    reg_file.clock();
    reg_file.set_read_addresses([Bits::from(2u8).resize(), Bits::from(2u8).resize()]);

    reg_file.disable();

    assert_eq!(&reg_file.read_outputs[0], &Bits::from(0u8));
    assert_eq!(&reg_file.read_outputs[1], &Bits::from(0u8));
}

#[test]
fn test_enable_restores_read_values() {
    let mut reg_file = RegisterFile::default();
    reg_file.enable();

    reg_file.schedule_write(Bits::from(2u8).resize(), Bits::from(4u8).resize());
    reg_file.clock();
    reg_file.set_read_addresses([Bits::from(2).resize(), Bits::from(2).resize()]);
    dbg!(reg_file.read_outputs);
    reg_file.disable();

    reg_file.enable();

    assert_eq!(&reg_file.read_outputs[0], &Bits::from(4u8));
    assert_eq!(&reg_file.read_outputs[1], &Bits::from(4u8));
}
