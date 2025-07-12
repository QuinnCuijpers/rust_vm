use super::*;
#[test]
fn test_xor() {
    let alu = Alu::new(AluSettings::Xor);
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
        "00000001"
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
fn test_xnor() {
    let alu = Alu::new(AluSettings::Xnor);
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
        "11111110"
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
