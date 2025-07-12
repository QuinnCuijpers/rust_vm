use super::super::*;

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
