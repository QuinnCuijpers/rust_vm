use super::super::*;

#[allow(clippy::bool_assert_comparison)]
#[test]
fn test_indexing() {
    let bits = Bits::<8>::from(0b0000_0100u8);
    assert_eq!(bits[0], false);
    assert_eq!(bits[2], true);
}

#[test]
fn test_partial_eq_and_copy() {
    let bits1 = Bits::<4>::from([true, false, true, false]);
    let bits2 = bits1;
    assert_eq!(bits1, bits2);
}

#[allow(clippy::clone_on_copy)]
#[test]
fn test_clone() {
    let bits1 = Bits::<4>::from([true, false, true, false]);
    let bits2 = bits1.clone();
    assert_eq!(bits1.bit_array, bits2.bit_array);
}

#[test]
fn test_display() {
    let bits = Bits::from(13u8);
    assert_eq!(format!("{:0>8b}", 13), bits.to_string())
}

#[test]
fn test_split_into_chunks() {
    let bits = Bits::<16>::from(0b0000_0000_0000_1111u16);
    let chunks = bits.split_into_chunks::<4>();
    assert_eq!(chunks.len(), 4);
    assert_eq!(chunks[0].to_string(), "0000");
    assert_eq!(chunks[1].to_string(), "0000");
    assert_eq!(chunks[2].to_string(), "0000");
    assert_eq!(chunks[3].to_string(), "1111");
}
