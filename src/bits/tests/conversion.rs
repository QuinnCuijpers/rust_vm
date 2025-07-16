use crate::bits::{Bits, BitsParseError};
use std::str::FromStr;

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
fn test_from_str_decimal_vs_binary_equivalence() {
    // "0101" binary is 5 in decimal
    let from_binary = Bits::<4>::from_str("0101").unwrap();
    let from_decimal = Bits::<4>::from_str("5").unwrap();

    assert_eq!(from_binary, from_decimal);
}

#[test]
fn test_try_from_unsigned() {
    let bits: Bits<8> = Bits::try_from_unsigned_number(255u8).unwrap();
    assert_eq!(
        bits.bit_array,
        [true, true, true, true, true, true, true, true]
    );

    let bits: Bits<4> = Bits::try_from_unsigned_number(15u8).unwrap();
    assert_eq!(bits.bit_array, [true, true, true, true]);

    let out_of_bounds = Bits::<4>::try_from_unsigned_number(16u8);
    assert_eq!(
        out_of_bounds,
        Err(BitsParseError::OutOfBounds {
            value: 16,
            max: (1 << 4) - 1
        })
    );
}

#[test]
fn test_from_ref_bits() {
    let bits = Bits::<8>::from(0b10101010u8);
    let ref_bits: &Bits<8> = &bits;
    assert_eq!(ref_bits.to_string(), "10101010");

    assert_eq!(u8::from(ref_bits), 0b10101010);
    assert_eq!(ref_bits.to_string(), "10101010");

    // Test that formatting a reference yields the same as owned
    assert_eq!(format!("{}", ref_bits), format!("{}", bits));
}

#[test]
fn test_from_bool_slice() {
    let bools: [bool; 4] = [true, false, true, false];
    let bits: Bits<4> = Bits::from(bools);
    assert_eq!(bits.bit_array, bools);

    // Test conversion back to slice
    let slice: &[bool] = &bits.bit_array;
    assert_eq!(slice, &bools);
}

#[test]
fn test_bool_slice_from_bits() {
    let bits: Bits<4> = Bits::from(10u8).resize();
    let bools: [bool; 4] = bits.into();
    let expected: [bool; 4] = [false, true, false, true];
    assert_eq!(bools, expected);

    // Test conversion back to Bits
    let bits_from_slice: Bits<4> = Bits::from(bools);
    assert_eq!(bits_from_slice, bits);
}
