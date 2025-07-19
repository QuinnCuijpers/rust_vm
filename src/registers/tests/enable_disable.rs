use crate::bits::Bits;

use super::super::*;
#[test]
fn disable_resets_read_values() {
    let mut reg_file = RegisterFile::default();

    reg_file.schedule_write((Bits::from(2u8).resize(), Bits::from(4u8).resize()));
    reg_file.clock();
    reg_file.set_read_addresses([Bits::from(2u8).resize(), Bits::from(2u8).resize()]);

    reg_file.enable(false);

    assert_eq!(&reg_file.read_outputs[0], &Bits::from(0u8));
    assert_eq!(&reg_file.read_outputs[1], &Bits::from(0u8));
}

#[test]
fn enable_restores_read_values() {
    let mut reg_file = RegisterFile::default();

    reg_file.schedule_write((Bits::from(2u8).resize(), Bits::from(4u8).resize()));
    reg_file.clock();
    reg_file.set_read_addresses([Bits::from(2).resize(), Bits::from(2).resize()]);
    reg_file.enable(false);

    reg_file.enable(true);

    assert_eq!(&reg_file.read_outputs[0], &Bits::from(4u8));
    assert_eq!(&reg_file.read_outputs[1], &Bits::from(4u8));
}
