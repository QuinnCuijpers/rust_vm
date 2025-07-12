use super::*;
#[test]
fn test_or() {
    let alu = Alu::new(AluSettings::Or);
    assert_bits!(
        alu.compute(bits_8!(0b00000000), bits_8!(0b00000000)),
        "00000000"
    );
    assert_bits!(
        alu.compute(bits_8!(0b00000001), bits_8!(0b00000001)),
        "00000001"
    );
    assert_bits!(
        alu.compute(bits_8!(0b00000010), bits_8!(0b00000011)),
        "00000011"
    );
    assert_bits!(
        alu.compute(bits_8!(0b00001111), bits_8!(0b00010000)),
        "00011111"
    );
    assert_bits!(
        alu.compute(bits_8!(0b11111111), bits_8!(0b00000000)),
        "11111111"
    );
    assert_bits!(
        alu.compute(bits_8!(0b10101010), bits_8!(0b01010101)),
        "11111111"
    );
}

#[test]
fn test_nor() {
    let alu = Alu::new(AluSettings::Nor);
    assert_bits!(
        alu.compute(bits_8!(0b00000000), bits_8!(0b00000000)),
        "11111111"
    );
    assert_bits!(
        alu.compute(bits_8!(0b00000001), bits_8!(0b00000001)),
        "11111110"
    );
    assert_bits!(
        alu.compute(bits_8!(0b00000010), bits_8!(0b00000011)),
        "11111100"
    );
    assert_bits!(
        alu.compute(bits_8!(0b00001111), bits_8!(0b00010000)),
        "11100000"
    );
    assert_bits!(
        alu.compute(bits_8!(0b11111111), bits_8!(0b00000000)),
        "00000000"
    );
    assert_bits!(
        alu.compute(bits_8!(0b10101010), bits_8!(0b01010101)),
        "00000000"
    );
}

#[test]
fn test_and() {
    let alu = Alu::new(AluSettings::And);
    assert_bits!(
        alu.compute(bits_8!(0b00000000), bits_8!(0b00000000)),
        "00000000"
    );
    assert_bits!(
        alu.compute(bits_8!(0b00000001), bits_8!(0b00000001)),
        "00000001"
    );
    assert_bits!(
        alu.compute(bits_8!(0b00000010), bits_8!(0b00000011)),
        "00000010"
    );
    assert_bits!(
        alu.compute(bits_8!(0b00001111), bits_8!(0b00010000)),
        "00000000"
    );
    assert_bits!(
        alu.compute(bits_8!(0b11111111), bits_8!(0b00000000)),
        "00000000"
    );
    assert_bits!(
        alu.compute(bits_8!(0b10101010), bits_8!(0b01010101)),
        "00000000"
    );
}

#[test]
fn test_nand() {
    let alu = Alu::new(AluSettings::Nand);
    assert_bits!(
        alu.compute(bits_8!(0b00000000), bits_8!(0b00000000)),
        "11111111"
    );
    assert_bits!(
        alu.compute(bits_8!(0b00000001), bits_8!(0b00000001)),
        "11111110"
    );
    assert_bits!(
        alu.compute(bits_8!(0b00000010), bits_8!(0b00000011)),
        "11111101"
    );
    assert_bits!(
        alu.compute(bits_8!(0b00001111), bits_8!(0b00010000)),
        "11111111"
    );
    assert_bits!(
        alu.compute(bits_8!(0b11111111), bits_8!(0b00000000)),
        "11111111"
    );
    assert_bits!(
        alu.compute(bits_8!(0b10101010), bits_8!(0b01010101)),
        "11111111"
    );
}

#[test]
fn test_implies() {
    let alu = Alu::new(AluSettings::Implies);
    assert_bits!(
        alu.compute(bits_8!(0b00000000), bits_8!(0b00000000)),
        "11111111"
    );
    assert_bits!(
        alu.compute(bits_8!(0b00000001), bits_8!(0b00000001)),
        "11111111"
    );
    assert_bits!(
        alu.compute(bits_8!(0b00000010), bits_8!(0b00000011)),
        "11111111"
    );
    assert_bits!(
        alu.compute(bits_8!(0b00001111), bits_8!(0b00010000)),
        "11110000"
    );
    assert_bits!(
        alu.compute(bits_8!(0b11111111), bits_8!(0b00000000)),
        "00000000"
    );
}

#[test]
fn test_nimplies() {
    let alu = Alu::new(AluSettings::Nimplies);
    assert_bits!(
        alu.compute(bits_8!(0b00000000), bits_8!(0b00000000)),
        "00000000"
    );
    assert_bits!(
        alu.compute(bits_8!(0b00000001), bits_8!(0b00000001)),
        "00000000"
    );
    assert_bits!(
        alu.compute(bits_8!(0b00000010), bits_8!(0b00000011)),
        "00000000"
    );
    assert_bits!(
        alu.compute(bits_8!(0b00001111), bits_8!(0b00010000)),
        "00001111"
    );
    assert_bits!(
        alu.compute(bits_8!(0b11111111), bits_8!(0b00000000)),
        "11111111"
    );
}
