use super::super::super::*;
use crate::error::VmError;
use std::fs::File;
use std::io::Write;

#[allow(clippy::panic)]
#[test]
fn unknown_operand() {
    let test_file = "unknown_operand.as";
    let mut file = File::create(test_file).unwrap();
    writeln!(file, "FOO r1 r2 r3").unwrap();
    drop(file);

    let result = parse_program(test_file);
    std::fs::remove_file(test_file).unwrap();

    match result.unwrap_err() {
        VmError::Parser(ParserError::InvalidInstruction(instr)) => assert_eq!(instr, "FOO"),
        _ => panic!("Expected InvalidInstruction error"),
    }
}
