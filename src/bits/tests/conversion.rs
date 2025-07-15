use super::super::*;

#[test]
fn test_to_usize() {
    let a = Bits::from(164u16);
    let b = 164usize;
    assert_eq!(a.to_usize(), b);
}

#[test]
fn test_u8_conversion() {
    let bits = Bits::<8>::from(0b1010_1010u8);
    assert_eq!(
        bits.bit_array,
        [false, true, false, true, false, true, false, true]
    );

    let back: u8 = bits.into();
    assert_eq!(back, 0b1010_1010);
}

#[allow(clippy::bool_assert_comparison)]
#[test]
fn test_u16_conversion() {
    let bits = Bits::<16>::from(0b1100_1010_0101_1010u16);
    assert_eq!(bits[0], false);
    assert_eq!(bits[1], true);
    assert_eq!(bits[15], true);
}

#[test]
fn test_signed_conversion() {
    let bits = Bits::<8>::from(-5i8);
    let back: i8 = bits.into();
    assert_eq!(back, -5);
}

#[test]
fn test_array_conversion() {
    let arr = [true, false, true, false];
    let bits = Bits::<4>::from(arr);
    assert_eq!(bits.bit_array, arr);
}

#[test]
fn test_parse_binary_string_big_endian() {
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
fn test_parse_decimal_string_little_endian() {
    // Parse decimal string "9" into 4-bit little-endian: 1001
    let bits = Bits::<4>::from_str("9").unwrap();

    let expected = Bits {
        bit_array: [true, false, false, true],
    }; // 1001
    assert_eq!(bits, expected);
}

#[test]
fn test_display_outputs_big_endian_string() {
    // Internally: little-endian [false, false, false, true] => binary 1000 => string "1000" (MSB first)
    let bits = Bits {
        bit_array: [false, false, false, true],
    }; // little-endian: 0 + 0*2 + 0*4 + 1*8 = 8

    let display = format!("{}", bits);
    assert_eq!(display, "1000");
}

#[test]
fn test_display_and_parse_roundtrip() {
    let original = Bits::<4>::from_str("1010").unwrap();
    let rendered = format!("{}", original);
    let reparsed = Bits::<4>::from_str(&rendered).unwrap();

    assert_eq!(original, reparsed);
}

#[test]
fn test_from_str_decimal_vs_binary_equivalence() {
    // "0101" binary is 5 in decimal
    let from_binary = Bits::<4>::from_str("0101").unwrap();
    let from_decimal = Bits::<4>::from_str("5").unwrap();

    assert_eq!(from_binary, from_decimal);
}
