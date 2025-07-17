use crate::{
    alu::{Alu, AluSettings},
    bits::Bits,
};

#[test]
fn rshift() {
    let mut alu = Alu::new(AluSettings::Rshift);
    let a = Bits::from(4u8);
    let b = Bits::from(0u8);
    assert_bits!(alu.compute(a, b), "00000010");
    assert_eq!(u8::from(alu.compute(a, b)), 2);
}
