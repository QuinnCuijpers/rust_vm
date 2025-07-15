use crate::{alu::AluSettings, bits::Bits};

#[test]
fn add() {
    let set = AluSettings::from_bits(Bits::from(2u8).resize());
    assert_eq!(set, AluSettings::Add);
}

#[test]
fn sub() {
    let set = AluSettings::from_bits(Bits::from(3u8).resize());
    assert_eq!(set, AluSettings::Sub);
}

#[test]
fn and() {
    let set = AluSettings::from_bits(Bits::from(4u8).resize());
    assert_eq!(set, AluSettings::And);
}

#[test]
fn nor() {
    let set = AluSettings::from_bits(Bits::from(5u8).resize());
    assert_eq!(set, AluSettings::Nor);
}

#[test]
fn xor() {
    let set = AluSettings::from_bits(Bits::from(6u8).resize());
    assert_eq!(set, AluSettings::Xor);
}

#[test]
fn rshift() {
    let set = AluSettings::from_bits(Bits::from(7u8).resize());
    assert_eq!(set, AluSettings::Rshift);
}

#[test]
#[should_panic]
fn not_yet_implemented() {
    let set = AluSettings::from_bits(Bits::from(15u8).resize());
}
