use crate::bits::Bits;

#[test]
fn to_usize() {
    let a = Bits::from(164u16);
    let b = 164usize;
    assert_eq!(a.to_usize(), b);
}

#[test]
fn to_usize_0() {
    let bits = Bits::<8>::from(0u8);
    assert_eq!(bits.to_usize(), 0);
}

#[test]
fn u8_conversion() {
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
fn u16_conversion() {
    let bits = Bits::<16>::from(0b1100_1010_0101_1010u16);
    assert_eq!(bits[0], false);
    assert_eq!(bits[1], true);
    assert_eq!(bits[15], true);
}
