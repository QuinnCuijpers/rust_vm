#![allow(clippy::panic)]
use super::super::super::*;
use crate::{bits::BitsParseError, error::VmError};

#[test]
fn parse_register_string_value_too_large() {
    let test_file = "reg_too_large.as";
    std::fs::write(test_file, "ADD r16 r1 r2").unwrap();
    let err = parse_program(test_file).unwrap_err();
    match err {
        VmError::Bits(BitsParseError::OutOfBounds { value, max }) => {
            assert_eq!(value, 16);
            assert_eq!(max, 15);
        }
        _ => panic!("Expected OutOfBounds error"),
    }
    std::fs::remove_file(test_file).unwrap();
}

#[test]
fn parse_register_string_negative_value() {
    let test_file = "reg_negative.as";
    std::fs::write(test_file, "ADD r-1 r1 r2").unwrap();
    let err = parse_program(test_file).unwrap_err();
    match err {
        VmError::Parser(ParserError::InvalidInstruction(ref s)) => assert_eq!(s, "r-1"),
        _ => panic!("Expected InvalidInstruction error"),
    }
    std::fs::remove_file(test_file).unwrap();
}

#[test]
fn parse_register_string_non_numeric_value() {
    let test_file = "reg_non_numeric.as";
    std::fs::write(test_file, "ADD rX r1 r2").unwrap();
    let err = parse_program(test_file).unwrap_err();
    match err {
        VmError::Bits(BitsParseError::Number { .. }) => (),
        _ => panic!("Expected Number error"),
    }
    std::fs::remove_file(test_file).unwrap();
}

#[test]
fn parse_register_string_invalid_prefix() {
    let test_file = "reg_invalid_prefix.as";
    std::fs::write(test_file, "ADD x1 r1 r2").unwrap();
    let err = parse_program(test_file).unwrap_err();
    match err {
        VmError::Parser(ParserError::InvalidInstruction(ref s)) => assert_eq!(s, "x1"),
        _ => panic!("Expected InvalidInstruction error"),
    }
    std::fs::remove_file(test_file).unwrap();
}

#[test]
fn parse_register_string_too_short() {
    let test_file = "reg_too_short.as";
    std::fs::write(test_file, "ADD r r1 r2").unwrap();
    let err = parse_program(test_file).unwrap_err();
    match err {
        VmError::Parser(ParserError::InvalidInstruction(ref s)) => assert_eq!(s, "r"),
        _ => panic!("Expected InvalidInstruction error"),
    }
    std::fs::remove_file(test_file).unwrap();
}

#[test]
fn ldi_register_parse_fail() {
    let test_file = "ldi_register_fail.as";
    std::fs::write(test_file, "LDI r16 8").unwrap();
    let err = parse_program(test_file).unwrap_err();
    match err {
        VmError::Bits(BitsParseError::OutOfBounds { value, max }) => {
            assert_eq!(value, 16);
            assert_eq!(max, 15);
        }
        _ => panic!("Expected OutOfBounds error"),
    }
    std::fs::remove_file(test_file).unwrap();
}

#[test]
fn rsh_invalid_first_operand() {
    let test_file = "rsh_invalid_first_operand.as";
    std::fs::write(test_file, "RSH rX r1").unwrap();
    let err = parse_program(test_file).unwrap_err();
    match err {
        VmError::Bits(BitsParseError::Number { .. }) => (),
        _ => panic!("Expected Number error"),
    }
    std::fs::remove_file(test_file).unwrap();
}
