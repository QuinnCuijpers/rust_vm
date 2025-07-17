use crate::bits::Bits;

#[test]
fn from_array() {
    let arr = [true, false, true, false, true, false, true, false];
    let bits = Bits::<8>::from(arr);
    assert_eq!(bits.bit_array, arr);
}

#[test]
fn into_array() {
    let bits = Bits::<8>::from(0b1010_1010u8);
    let arr: [bool; 8] = bits.into();
    assert_eq!(arr, [false, true, false, true, false, true, false, true]);
}
