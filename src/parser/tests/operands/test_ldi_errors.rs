#![allow(clippy::panic)]
use std::fs::File;
use std::io::Write;

use super::super::super::*;
use crate::{bits::BitsParseError, error::VmError};

#[test]
fn ldi_missing_value_operand() {
    let test_file = "ldi_missing_value.as";
    std::fs::write(test_file, "LDI r1").unwrap();
    let err = parse_program(test_file).unwrap_err();
    match err {
        VmError::Parser(ParserError::MissingOperand(ref line)) => {
            assert!(line.contains("LDI r1"));
        }
        _ => panic!("Expected MissingOperand error"),
    }
    std::fs::remove_file(test_file).unwrap();
}

#[test]
fn ldi_too_many_operands() {
    let test_file = "test_ldi_too_many_operands.as";
    let mut file = File::create(test_file).unwrap();
    writeln!(file, "LDI r1 42 extra").unwrap();
    drop(file);
    let err = parse_program(test_file).unwrap_err();
    match err {
        VmError::Parser(ParserError::MissingOperand(ref line)) => {
            assert!(line.contains("LDI r1 42 extra"))
        }
        _ => panic!("Expected MissingOperand error"),
    }
    std::fs::remove_file(test_file).unwrap();
}

#[test]
fn ldi_only_instruction() {
    let test_file = "test_ldi_only_instruction.as";
    let mut file = File::create(test_file).unwrap();
    writeln!(file, "LDI").unwrap();
    drop(file);
    let err = parse_program(test_file).unwrap_err();
    match err {
        VmError::Parser(ParserError::MissingOperand(ref line)) => assert!(line.contains("LDI")),
        _ => panic!("Expected MissingOperand error"),
    }
    std::fs::remove_file(test_file).unwrap();
}

#[test]
fn ldi_invalid_value_operand() {
    let test_file = "ldi_invalid_value.as";
    std::fs::write(test_file, "LDI r1 na").unwrap();
    let err = parse_program(test_file).unwrap_err();
    match err {
        VmError::Bits(BitsParseError::Number { .. }) => (),
        _ => panic!("Expected Number error"),
    }
    std::fs::remove_file(test_file).unwrap();
}

#[test]
fn ldi_value_too_large() {
    let test_file = "ldi_value_too_large.as";
    std::fs::write(test_file, "LDI r1 300").unwrap();
    let err = parse_program(test_file).unwrap_err();
    match err {
        VmError::Bits(BitsParseError::OutOfBounds { value, max }) => {
            assert_eq!(value, 300);
            assert_eq!(max, 255);
        }
        _ => panic!("Expected OutOfBounds error"),
    }
    std::fs::remove_file(test_file).unwrap();
}
