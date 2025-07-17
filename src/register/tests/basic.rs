use std::str::FromStr;

use super::super::*;
use crate::assert_bits;
#[test]
fn write_and_clock_propagates_to_both_banks() {
    let mut reg_file = RegisterFile::default();
    reg_file.enable();

    let data = Bits::try_from_unsigned_number(7u8).unwrap();
    let address = Bits::try_from_unsigned_number(1u8).unwrap();

    reg_file.schedule_write(address, data);
    reg_file.clock();

    assert_eq!(&reg_file.register_banks[0][address.to_usize()], &data);
    assert_eq!(&reg_file.register_banks[1][address.to_usize()], &data);
}

#[test]
fn update_read_reads_correct_value() {
    let mut reg_file = RegisterFile::default();
    reg_file.enable();

    let data = Bits::try_from_unsigned_number(7u8).unwrap();
    let address = Bits::try_from_unsigned_number(1u8).unwrap();

    reg_file.schedule_write(address, data);
    reg_file.clock();
    reg_file.set_read_addresses([address, Bits::from_str("0").unwrap()]);

    let read_value = reg_file.read_outputs[0];
    assert_bits!(read_value, "00000111");
}

#[test]
fn sequential_write_and_read() {
    let mut reg_file = RegisterFile::default();
    reg_file.enable();

    let data = Bits::try_from_unsigned_number(4u8).unwrap();
    let address: Bits<4> = Bits::try_from_unsigned_number(2u8).unwrap();

    reg_file.schedule_write(address, data);
    reg_file.clock();

    reg_file.set_read_addresses([Bits::from_str("2").unwrap(), Bits::from_str("1").unwrap()]);

    assert_eq!(&reg_file.register_banks[0][address.to_usize()], &data);
    assert_eq!(&reg_file.read_outputs[0], &data);
}

#[test]
fn dual_read_same_value() {
    let mut reg_file = RegisterFile::default();
    reg_file.enable();

    let data: Bits<8> = Bits::from(4u8);
    let address: Bits<4> = Bits::try_from_unsigned_number(2u8).unwrap();

    reg_file.schedule_write(address, data);
    reg_file.clock();

    reg_file.set_read_addresses([
        Bits::try_from_unsigned_number(2u8).unwrap(),
        Bits::try_from_unsigned_number(2u8).unwrap(),
    ]);
    assert_eq!(&reg_file.read_outputs[1], &data);
}
