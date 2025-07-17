use super::super::*;
#[test]
fn parse_instruction_valid() {
    assert_eq!(parse_instruction("ADD").unwrap().to_string(), "0010");
    assert_eq!(parse_instruction("SUB").unwrap().to_string(), "0011");
    assert_eq!(parse_instruction("AND").unwrap().to_string(), "0100");
    assert_eq!(parse_instruction("NOR").unwrap().to_string(), "0101");
    assert_eq!(parse_instruction("XOR").unwrap().to_string(), "0110");
    assert_eq!(parse_instruction("RSH").unwrap().to_string(), "0111");
}

#[test]
fn parse_instruction_invalid() {
    let result = parse_instruction("FOO");
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Invalid instruction"));
}
