use crate::bits::Bits;
use std::str::FromStr;

#[test]
fn from_str() {
    let bits = Bits::<8>::from_str("10101010").unwrap();
    assert_eq!(
        bits.bit_array,
        [false, true, false, true, false, true, false, true]
    );
}

#[test]
fn to_string() {
    let bits = Bits::<8>::from(0b1010_1010u8);
    assert_eq!(bits.to_string(), "10101010");
}

#[test]
fn parse_binary_string_big_endian() {
    // Parse "1100" as binary string (interpreted big-endian visually)
    let bits = Bits::<4>::from_str("1100").unwrap();

    // Since storage is little-endian, bits[0] is LSB
    // "1100" => [false, false, true, true]
    let expected = Bits {
        bit_array: [false, false, true, true],
    };
    assert_eq!(bits, expected);
}

#[test]
fn parse_decimal_string_little_endian() {
    // Parse decimal string "9" into 4-bit little-endian: 1001
    let bits = Bits::<4>::from_str("9").unwrap();

    let expected = Bits {
        bit_array: [true, false, false, true],
    }; // 1001
    assert_eq!(bits, expected);
}

#[test]
fn from_str_decimal_vs_binary_equivalence() {
    // "0101" binary is 5 in decimal
    let from_binary = Bits::<4>::from_str("0101").unwrap();
    let from_decimal = Bits::<4>::from_str("5").unwrap();

    assert_eq!(from_binary, from_decimal);
}
