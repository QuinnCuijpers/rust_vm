use std::str::FromStr;

use crate::bits::Bits;

#[test]
fn parse_invalid_binary_string() {
    // Try to parse a string with invalid characters for binary
    let out = Bits::<8>::from_str("10a01001").unwrap_err();
    assert_eq!(
        out,
        crate::Error::Bits(crate::bits::BitsParseError::Character { character: 'a' })
    );
}

#[test]
fn parse_too_long_string() {
    // Try to parse a string that's too long for Bits<8>
    let out = Bits::<8>::from_str("101010101");
    assert_eq!(
        out,
        Err(crate::Error::Bits(crate::bits::BitsParseError::Length {
            expected: 8,
            found: 9,
            string: "101010101".to_string()
        }))
    );
}

#[test]
fn parse_too_large_number() {
    // Try to parse a number that exceeds the maximum for Bits<8>
    let out = Bits::<8>::from_str("256");
    assert_eq!(
        out,
        Err(crate::Error::Bits(
            crate::bits::BitsParseError::OutOfBounds {
                value: 256,
                max: 255
            }
        ))
    );
}

#[test]
#[should_panic]
fn parse_empty_string() {
    // Try to parse an empty string
    let _ = Bits::<8>::from_str("").unwrap();
}
