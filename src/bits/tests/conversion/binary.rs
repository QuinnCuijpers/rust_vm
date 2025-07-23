use crate::assert_bits;

#[test]
fn bits_from_binary() {
    use crate::bits::Bits;
    use std::str::FromStr;
    let b_str = "0b11";
    let b: Bits<8> = Bits::<8>::from_str(b_str).unwrap();
    assert_bits!(b, "00000011");
}
