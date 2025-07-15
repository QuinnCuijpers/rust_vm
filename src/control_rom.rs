use crate::alu::AluSettings;
use crate::register::RegisterFile;
use crate::{bits::Bits, parser};
use std::{fs, str::FromStr};

fn execute_as(file_path: &str) {
    let mut alu = crate::alu::Alu::default();
    let mut rb: Vec<u8> = Vec::from([0, 7, 8, 4]);
    rb.extend_from_slice(&[0u8; 12]);
    let rb: Vec<_> = rb.iter().map(|val| Bits::from(*val)).collect();
    let mut rb_arr = [Bits::from(0u8); 16];
    for (i, val) in rb.iter().enumerate() {
        rb_arr[i] = *val;
    }
    let mut reg_file = RegisterFile::new(rb_arr);
    parser::parse_program(file_path).expect("Failed to parse file");
    let path = std::path::Path::new(file_path).with_extension("mc");
    let programm = fs::read_to_string(path).expect("Failed to read file");

    let content_bits: Vec<Vec<Bits<4>>> = programm
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|s| Bits::from_str(s).unwrap())
                .collect()
        })
        .collect();

    for machine_code in content_bits {
        let parts = machine_code;
        alu.set_setting(AluSettings::from_bits(parts[0])); // TODO: also add enable check
        let a = parts[1];
        let b = parts[2];
        let write_address = parts[3];
        reg_file.set_read_addresses([a, b]);
        let result = alu.compute(reg_file.read_outputs[0], reg_file.read_outputs[1]);
        reg_file.schedule_write(write_address, result);
        reg_file.clock();
    }
    for (i, val) in reg_file.register_banks[0].iter().enumerate() {
        println!("Register {i}: {}", val.to_usize());
    }
}

mod tests {
    use super::*;

    // TODO: update tests to cover correct calculations
    #[test]
    fn test_execute_as() {
        let file_path = "test.as";
        execute_as(file_path);
    }
}
