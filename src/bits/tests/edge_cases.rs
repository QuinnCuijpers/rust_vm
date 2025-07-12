use super::super::*;

#[test]
fn test_min_u8_value() {
    let bits = Bits::<8>::from(0u8);
    assert_eq!(bits.bit_array, [false; 8]);
}

#[test]
fn test_max_u8_value() {
    let bits = Bits::<8>::from(255u8);
    assert_eq!(bits.bit_array, [true; 8]);
}

#[allow(clippy::bool_assert_comparison)]
#[test]
fn test_signed_edge_cases() {
    let bits = Bits::<8>::from(127i8);
    assert_eq!(bits[7], false); // sign bit
    let bits = Bits::<8>::from(-128i8);
    assert_eq!(bits[7], true); // sign bit
}
