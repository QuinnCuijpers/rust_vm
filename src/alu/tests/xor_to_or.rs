use super::*;
#[test]
fn or() {
    let mut alu = Alu::new(AluSettings::Or);
    assert_bits!(
        alu.compute(bits_8!(0b00000010), bits_8!(0b00000011)),
        "00000011"
    );
    assert_bits!(
        alu.compute(bits_8!(0b10101010), bits_8!(0b01010101)),
        "11111111"
    );
    assert_bits!(
        alu.compute(bits_8!(0b11110000), bits_8!(0b00001111)),
        "11111111"
    );
}

#[test]
fn nor() {
    let mut alu = Alu::new(AluSettings::Nor);
    assert_bits!(
        alu.compute(bits_8!(0b00000010), bits_8!(0b00000011)),
        "11111100"
    );
    assert_bits!(
        alu.compute(bits_8!(0b10101010), bits_8!(0b01010101)),
        "00000000"
    );
    assert_bits!(
        alu.compute(bits_8!(0b11110000), bits_8!(0b00001111)),
        "00000000"
    );
}

#[test]
fn and() {
    let mut alu = Alu::new(AluSettings::And);
    assert_bits!(
        alu.compute(bits_8!(0b00000010), bits_8!(0b00000011)),
        "00000010"
    );
    assert_bits!(
        alu.compute(bits_8!(0b10101010), bits_8!(0b01010101)),
        "00000000"
    );
    assert_bits!(
        alu.compute(bits_8!(0b11110000), bits_8!(0b00001111)),
        "00000000"
    );
}

#[test]
fn nand() {
    let mut alu = Alu::new(AluSettings::Nand);
    assert_bits!(
        alu.compute(bits_8!(0b00000010), bits_8!(0b00000011)),
        "11111101"
    );
    assert_bits!(
        alu.compute(bits_8!(0b10101010), bits_8!(0b01010101)),
        "11111111"
    );
    assert_bits!(
        alu.compute(bits_8!(0b11110000), bits_8!(0b00001111)),
        "11111111"
    );
}

#[test]
fn implies() {
    let mut alu = Alu::new(AluSettings::Implies);
    assert_bits!(
        alu.compute(bits_8!(0b00000010), bits_8!(0b00000011)),
        "11111111"
    );
    assert_bits!(
        alu.compute(bits_8!(0b10101010), bits_8!(0b01010101)),
        "01010101"
    );
    assert_bits!(
        alu.compute(bits_8!(0b11110000), bits_8!(0b00001111)),
        "00001111"
    );
}

#[test]
fn nimplies() {
    let mut alu = Alu::new(AluSettings::Nimplies);
    assert_bits!(
        alu.compute(bits_8!(0b00000010), bits_8!(0b00000011)),
        "00000000"
    );
    assert_bits!(
        alu.compute(bits_8!(0b10101010), bits_8!(0b01010101)),
        "10101010"
    );
    assert_bits!(
        alu.compute(bits_8!(0b11110000), bits_8!(0b00001111)),
        "11110000"
    );
}
