use super::super::*;
#[test]
fn test_zero_index_is_ignored() {
    let mut reg_file = RegisterFile::default();
    reg_file.enable();

    reg_file.schedule_write(0, Bits::from(123u8));
    reg_file.clock();

    // Register at index 0 should remain 0
    assert_eq!(reg_file.register_banks[0][0], Bits::from(0u8));
    assert_eq!(reg_file.register_banks[1][0], Bits::from(0u8));
}

#[test]
fn test_out_of_bounds_index_is_ignored() {
    let mut reg_file = RegisterFile::default();
    reg_file.enable();

    let out_of_bounds = 16; // Valid indices are 1..=15
    reg_file.schedule_write(out_of_bounds, Bits::from(42u8));
    reg_file.clock();

    for bank in &reg_file.register_banks {
        for reg in bank.iter() {
            assert_ne!(*reg, Bits::from(42u8));
        }
    }
}

#[test]
fn test_read_zero_address_no_update() {
    let mut reg_file = RegisterFile::default();
    reg_file.enable();

    reg_file.set_read_addresses([0, 0]);

    assert_eq!(reg_file.read_outputs[0], Bits::from(0u8));
    assert_eq!(reg_file.read_outputs[1], Bits::from(0u8));
}

#[test]
fn test_read_out_of_bounds_address_no_update() {
    let mut reg_file = RegisterFile::default();
    reg_file.enable();

    reg_file.set_read_addresses([16, 17]);

    assert_eq!(reg_file.read_outputs[0], Bits::from(0u8));
    assert_eq!(reg_file.read_outputs[1], Bits::from(0u8));
}
