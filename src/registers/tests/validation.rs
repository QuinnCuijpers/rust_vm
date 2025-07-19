use crate::bits::Bits;
use std::str::FromStr;

use super::super::*;
#[test]
fn zero_index_is_ignored() {
    let mut reg_file = RegisterFile::default();

    reg_file.schedule_write((Bits::from_str("0").unwrap(), Bits::from_str("15").unwrap()));
    reg_file.clock();

    // Register at index 0 should remain 0
    assert_eq!(reg_file.register_banks[0][0], Bits::from(0u8));
    assert_eq!(reg_file.register_banks[1][0], Bits::from(0u8));
}

#[test]
fn read_zero_address_no_update() {
    let mut reg_file = RegisterFile::default();

    reg_file.set_read_addresses([Bits::from_str("0").unwrap(), Bits::from_str("0").unwrap()]);

    assert_eq!(reg_file.read_outputs[0], Bits::from(0u8));
    assert_eq!(reg_file.read_outputs[1], Bits::from(0u8));
}

#[test]
fn read_from_0() {
    let mut reg_file = RegisterFile::default();

    reg_file.schedule_write((Bits::from_str("0").unwrap(), Bits::from_str("15").unwrap()));
    reg_file.clock();

    // Register at index 0 should remain 0
    assert_eq!(reg_file.read_outputs[0], Bits::from(0u8));
    assert_eq!(reg_file.read_outputs[0], Bits::from(0u8));
}
