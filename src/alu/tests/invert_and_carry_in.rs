use super::*;

#[test]
fn test_addition() {
    let alu = Alu::new(AluSettings::Add);
    assert_bits!(alu.compute(bits_8!(0), bits_8!(0)), "00000000");
    assert_bits!(alu.compute(bits_8!(1), bits_8!(1)), "00000010");
    assert_bits!(alu.compute(bits_8!(2), bits_8!(3)), "00000101");
    assert_bits!(alu.compute(bits_8!(15), bits_8!(16)), "00011111");
}

#[test]
fn test_subtraction() {
    let alu = Alu::new(AluSettings::Sub);
    assert_bits!(alu.compute(bits_8!(0), bits_8!(0)), "00000000");
    assert_bits!(alu.compute(bits_8!(5), bits_8!(3)), "00000010");
    assert_bits!(alu.compute(bits_8!(10), bits_8!(5)), "00000101");
    assert_bits!(alu.compute(bits_8!(128), bits_8!(128)), "00000000");
    assert_bits!(alu.compute(bits_8!(255), bits_8!(1)), "11111110");
}

#[test]
fn test_addition_edge_cases() {
    let alu = Alu::new(AluSettings::Add);
    assert_bits!(alu.compute(bits_8!(0), bits_8!(255)), "11111111");
    assert_bits!(alu.compute(bits_8!(255), bits_8!(0)), "11111111");
    assert_bits!(alu.compute(bits_8!(127), bits_8!(1)), "10000000");
    assert_bits!(alu.compute(bits_8!(200), bits_8!(55)), "11111111");
}

#[test]
fn test_subtraction_edge_cases() {
    let alu = Alu::new(AluSettings::Sub);
    assert_bits!(alu.compute(bits_8!(255), bits_8!(255)), "00000000");
    assert_bits!(alu.compute(bits_8!(1), bits_8!(255)), "00000010");
    assert_bits!(alu.compute(bits_8!(98), bits_8!(100)), "11111110");
    assert_bits!(alu.compute(bits_8!(0), bits_8!(255)), "00000001");
}
