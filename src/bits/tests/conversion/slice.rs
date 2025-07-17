use crate::bits::Bits;

#[test]
fn from_bool_slice() {
    let bools: [bool; 4] = [true, false, true, false];
    let bits: Bits<4> = Bits::from(bools);
    assert_eq!(bits.bit_array, bools);

    // Test conversion back to slice
    let slice: &[bool] = &bits.bit_array;
    assert_eq!(slice, &bools);
}

#[test]
fn bool_slice_from_bits() {
    let bits: Bits<4> = Bits::from(10u8).resize();
    let bools: [bool; 4] = bits.into();
    let expected: [bool; 4] = [false, true, false, true];
    assert_eq!(bools, expected);

    // Test conversion back to Bits
    let bits_from_slice: Bits<4> = Bits::from(bools);
    assert_eq!(bits_from_slice, bits);
}
