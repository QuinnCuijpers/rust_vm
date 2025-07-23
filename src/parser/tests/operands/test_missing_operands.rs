use super::super::super::*;
use std::fs::File;
use std::io::Write;

#[test]
fn parse_program_missing_operand() {
    let test_file = "missing_operand.as";
    let mut file = File::create(test_file).unwrap();
    writeln!(file, "ADD r1").unwrap();
    drop(file);
    let result = parse_program(test_file);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Missing operand"));
    std::fs::remove_file(test_file).unwrap();
}

#[test]
fn parse_program_missing_write_operand() {
    let test_file = "missing_write_operand.as";
    let mut file = File::create(test_file).unwrap();
    writeln!(file, "ADD r1 r2").unwrap();
    drop(file);
    let result = parse_program(test_file);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Missing operand"));
    std::fs::remove_file(test_file).unwrap();
}
