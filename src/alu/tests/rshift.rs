use super::*;
#[test]
fn test_rshift() {
    let alu = Alu::new(AluSettings::Rshift);
    let a = Bits::from(0u8);
    let b = Bits::from(4u8);
    print!("Testing right shift for {:?}", b);
    assert_bits!(alu.compute(a, b), "00000010");
    assert_eq!(u8::from(alu.compute(a, b)), 2);
}
