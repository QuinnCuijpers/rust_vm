use super::super::*;

#[allow(clippy::bool_assert_comparison)]
#[test]
fn indexing() {
    let bits = Bits::<8>::from(0b0000_0100u8);
    assert_eq!(bits[0], false);
    assert_eq!(bits[2], true);
}

#[test]
fn partial_eq_and_copy() {
    let bits1 = Bits::<4>::from([true, false, true, false]);
    let bits2 = bits1;
    assert_eq!(bits1, bits2);
}

#[allow(clippy::clone_on_copy)]
#[test]
fn clone() {
    let bits1 = Bits::<4>::from([true, false, true, false]);
    let bits2 = bits1.clone();
    assert_eq!(bits1.bit_array, bits2.bit_array);
}

#[test]
fn display() {
    let bits = Bits::from(13u8);
    assert_eq!(format!("{:0>8b}", 13), bits.to_string())
}

#[test]
fn split_into_chunks() {
    let bits = Bits::<16>::from(0b0000_0000_0000_1111u16);
    let chunks = bits.split_into_chunks::<4>();
    assert_eq!(chunks.len(), 4);
    assert_eq!(chunks[0].to_string(), "0000");
    assert_eq!(chunks[1].to_string(), "0000");
    assert_eq!(chunks[2].to_string(), "0000");
    assert_eq!(chunks[3].to_string(), "1111");
}

#[test]
fn add() {
    let bits1 = Bits::<8>::from(0b0000_0011u8); // 3 in binary
    let bits2 = Bits::<8>::from(0b0000_0101u8); // 5 in binary
    let result = bits1 + bits2;
    assert_eq!(result, Bits::<8>::from(0b0000_1000u8)); // 8 in binary
}

#[test]
fn sub() {
    let bits1 = Bits::<8>::from(0b0000_0110u8);
    let bits2 = Bits::<8>::from(0b0000_0011u8);
    let result = bits1 - bits2;
    assert_eq!(result, Bits::<8>::from(0b0000_0011u8));
}

#[test]
fn subassign() {
    let mut bits1 = Bits::<8>::from(0b0000_0110u8); // 6 in binary
    let bits2 = Bits::<8>::from(0b0000_0011u8); // 3 in binary
    bits1 -= bits2;
    assert_eq!(bits1, Bits::<8>::from(0b0000_0011u8)); // 3 in binary
}

#[test]
fn addassign() {
    let mut bits1 = Bits::<8>::from(0b0000_0011u8); // 3 in binary
    let bits2 = Bits::<8>::from(0b0000_0101u8); // 5 in binary
    bits1 += bits2;
    assert_eq!(bits1, Bits::<8>::from(0b0000_1000u8)); // 8 in binary
}
