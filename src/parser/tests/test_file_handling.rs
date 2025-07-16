use super::super::*;
use std::fs::File;
use std::io::Write;

#[test]
fn test_parse_program() {
    let result = parse_program("test.as");
    assert!(result.is_ok());
}

#[test]
fn test_parse_program_file_not_found() {
    let result = parse_program("nonexistent_file.as");
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("File not found"));
}

#[test]
fn test_parse_program_with_empty_lines_and_short_lines() {
    let test_file = "empty_and_short_lines.as";
    let mut file = File::create(test_file).unwrap();
    writeln!(file).unwrap(); // empty line
    writeln!(file, "ADD r1 r2 r3").unwrap(); // valid
    writeln!(file, "   ").unwrap(); // whitespace line
    writeln!(file, "SUB").unwrap(); // too short
    writeln!(file, "RSH r1 r2").unwrap(); // valid RSH
    writeln!(file, "AND r1").unwrap(); // too short
    drop(file);

    let result = parse_program(test_file);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    // Should fail on the first short line ("SUB")
    assert!(err.contains("Missing operand"));

    std::fs::remove_file(test_file).unwrap();
}
