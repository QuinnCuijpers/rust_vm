use crate::bits::Bits;

#[test]
fn i8_conversion() {
    let bits = Bits::<8>::from(-42i8);
    assert_eq!(
        bits.bit_array,
        [false, true, true, false, true, false, true, true]
    );
    let back: i8 = bits.into();
    assert_eq!(back, -42);
}

#[test]
fn i16_conversion() {
    let bits = Bits::<16>::from(-12345i16);
    let back: i16 = bits.into();
    assert_eq!(back, -12345);
}
