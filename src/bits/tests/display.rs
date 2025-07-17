use std::str::FromStr;

use crate::bits::{Bits, BitsParseError};

#[test]
fn display_outputs_big_endian_string() {
    // Internally: little-endian [false, false, false, true] => binary 1000 => string "1000" (MSB first)
    let bits = Bits {
        bit_array: [false, false, false, true],
    }; // little-endian: 0 + 0*2 + 0*4 + 1*8 = 8

    let display = format!("{bits}");
    assert_eq!(display, "1000");
}

#[test]
fn display_and_parse_roundtrip() {
    let original = Bits::<4>::from_str("1010").unwrap();
    let rendered = format!("{original}");
    let reparsed = Bits::<4>::from_str(&rendered).unwrap();

    assert_eq!(original, reparsed);
}

#[test]
fn bits_parse_error_display() {
    let error = BitsParseError::Length {
        expected: 4,
        found: 5,
    };
    assert_eq!(error.to_string(), "Invalid length: expected 4, found 5");

    let error = BitsParseError::Character { character: 'x' };
    assert_eq!(error.to_string(), "Invalid character: 'x'");

    let error = BitsParseError::Number {
        source: "invalid".parse::<u32>().unwrap_err(),
    };
    assert_eq!(
        error.to_string(),
        "Invalid number: invalid digit found in string"
    );

    let error = BitsParseError::OutOfBounds { value: 16, max: 15 };
    assert_eq!(error.to_string(), "Value 16 is out of bounds (max: 15)");
}
