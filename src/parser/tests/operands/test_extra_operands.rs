use super::super::super::*;
use std::fs::File;
use std::io::Write;

#[test]
fn parse_program_nop_with_extra_operands() {
    let test_file = "nop_with_extra_operands.as";
    let mut file = File::create(test_file).unwrap();
    writeln!(file, "NOP r1").unwrap(); // NOP should not have operands
    drop(file);
    let result = parse_program(test_file);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Missing operand"));
    std::fs::remove_file(test_file).unwrap();
}
