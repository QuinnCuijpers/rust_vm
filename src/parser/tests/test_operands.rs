use super::super::*;
use std::fs::File;
use std::io::Write;

#[test]
fn test_parse_program_missing_operand() {
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
fn test_parse_program_missing_write_operand() {
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

#[test]
fn test_parse_program_too_many_operands() {
    let test_file = "too_many_operands.as";
    let mut file = File::create(test_file).unwrap();
    writeln!(file, "ADD r1 r2 r3 r4").unwrap();
    drop(file);

    let result = parse_program(test_file);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Missing operand"));

    std::fs::remove_file(test_file).unwrap();
}

#[test]
fn test_parse_program_rsh_missing_operand() {
    let test_file = "rsh_missing_operand.as";
    let mut file = File::create(test_file).unwrap();
    writeln!(file, "RSH r1").unwrap();
    drop(file);

    let result = parse_program(test_file);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Missing operand"));

    std::fs::remove_file(test_file).unwrap();
}

#[test]
fn test_parse_program_rsh_three_operands() {
    let test_file = "rsh_three_operands.as";
    let mut file = File::create(test_file).unwrap();
    writeln!(file, "RSH r1 r2 r3").unwrap();
    drop(file);

    let result = parse_program(test_file);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Missing operand"));

    std::fs::remove_file(test_file).unwrap();
}

#[test]
fn test_parse_program_rsh_missing_first_operand() {
    let test_file = "rsh_missing_first_operand.as";
    let mut file = File::create(test_file).unwrap();
    writeln!(file, "RSH").unwrap();
    drop(file);

    let result = parse_program(test_file);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Missing operand"));

    std::fs::remove_file(test_file).unwrap();
}

#[test]
fn test_parse_program_nop_with_extra_operands() {
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

#[test]
fn test_unknown_operand() {
    let test_file = "unknown_operand.as";
    let mut file = File::create(test_file).unwrap();
    writeln!(file, "FOO r1 r2 r3").unwrap();
    drop(file);

    let result = parse_program(test_file);
    std::fs::remove_file(test_file).unwrap();

    match result.unwrap_err().downcast_ref::<ParserError>() {
        Some(ParserError::InvalidInstruction(instr)) => assert_eq!(instr, "FOO"),
        _ => panic!("Expected InvalidInstruction error"),
    }
}

#[test]
fn test_parse_register_string_invalid_prefix() {
    let err = parse_register_string("x1")
        .unwrap_err()
        .downcast::<ParserError>()
        .unwrap();
    match *err {
        ParserError::InvalidInstruction(ref s) => assert_eq!(s, "x1"),
        _ => panic!("Expected InvalidInstruction error"),
    }
}

#[test]
fn test_parse_register_string_too_short() {
    let err = parse_register_string("r")
        .unwrap_err()
        .downcast::<ParserError>()
        .unwrap();
    match *err {
        ParserError::InvalidInstruction(ref s) => assert_eq!(s, "r"),
        _ => panic!("Expected InvalidInstruction error"),
    }
}

#[test]
fn test_ldi_missing_value_operand() {
    let test_file = "test_ldi_missing_value.as";
    let mut file = File::create(test_file).unwrap();
    writeln!(file, "LDI r1").unwrap();
    drop(file);

    let err = parse_program(test_file)
        .unwrap_err()
        .downcast::<ParserError>()
        .unwrap();
    match *err {
        ParserError::MissingOperand(ref line) => assert!(line.contains("LDI r1")),
        _ => panic!("Expected MissingOperand error"),
    }

    std::fs::remove_file(test_file).unwrap();
}

#[test]
fn test_ldi_too_many_operands() {
    let test_file = "test_ldi_too_many_operands.as";
    let mut file = File::create(test_file).unwrap();
    writeln!(file, "LDI r1 42 extra").unwrap();
    drop(file);

    let err = parse_program(test_file)
        .unwrap_err()
        .downcast::<ParserError>()
        .unwrap();
    match *err {
        ParserError::MissingOperand(ref line) => assert!(line.contains("LDI r1 42 extra")),
        _ => panic!("Expected MissingOperand error"),
    }

    std::fs::remove_file(test_file).unwrap();
}

#[test]
fn test_ldi_only_instruction() {
    let test_file = "test_ldi_only_instruction.as";
    let mut file = File::create(test_file).unwrap();
    writeln!(file, "LDI").unwrap();
    drop(file);

    let err = parse_program(test_file)
        .unwrap_err()
        .downcast::<ParserError>()
        .unwrap();
    match *err {
        ParserError::MissingOperand(ref line) => assert!(line.contains("LDI")),
        _ => panic!("Expected MissingOperand error"),
    }

    std::fs::remove_file(test_file).unwrap();
}

#[test]
fn test_ldi_invalid_value_operand() {
    let test_file = "test_ldi_invalid_value.as";
    let mut file = File::create(test_file).unwrap();
    writeln!(file, "LDI r1 na").unwrap();
    drop(file);

    let err = parse_program(test_file).unwrap_err();
    let err_str = err.to_string();
    assert!(
        err_str.contains("Invalid number") || err_str.contains("ParseIntError"),
        "Error: {}",
        err_str
    );

    std::fs::remove_file(test_file).unwrap();
}

