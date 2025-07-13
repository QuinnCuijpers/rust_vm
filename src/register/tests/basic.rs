use super::super::*;
use crate::assert_bits;
#[test]
fn test_write_and_clock_propagates_to_both_banks() {
    let mut reg_file = RegisterFile::default();
    reg_file.enable();

    let data = Bits::from(7u8);
    let address = 1;

    reg_file.schedule_write(address, data);
    reg_file.clock();

    assert_eq!(&reg_file.register_banks[0][address], &data);
    assert_eq!(&reg_file.register_banks[1][address], &data);
}

#[test]
fn test_update_read_reads_correct_value() {
    let mut reg_file = RegisterFile::default();
    reg_file.enable();

    let data = Bits::from(7u8);
    let address = 1;

    reg_file.schedule_write(address, data);
    reg_file.clock();
    reg_file.set_read_addresses([address, 0]);

    let read_value = reg_file.read_outputs[0];
    assert_bits!(read_value, "00000111");
}

#[test]
fn test_sequential_write_and_read() {
    let mut reg_file = RegisterFile::default();
    reg_file.enable();

    let data = Bits::from(4u8);
    let address = 2;

    reg_file.schedule_write(address, data);
    reg_file.clock();

    reg_file.set_read_addresses([2, 1]);

    assert_eq!(&reg_file.register_banks[0][address], &data);
    assert_eq!(&reg_file.read_outputs[0], &data);
}

#[test]
fn test_dual_read_same_value() {
    let mut reg_file = RegisterFile::default();
    reg_file.enable();

    let data = Bits::from(4u8);
    let address = 2;

    reg_file.schedule_write(address, data);
    reg_file.clock();

    reg_file.set_read_addresses([2, 2]);
    assert_eq!(&reg_file.read_outputs[1], &data);
}
