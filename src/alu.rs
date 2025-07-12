use crate::bits::Bits;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Signal {
    Cancel,
    Generate,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct AluConfig {
    invert_a: bool,
    invert_b: bool,
    carry_in: bool,
    flood_carry: bool,
    xor_to_or: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum AluSettings {
    Add,
    Sub,
    Xor,
    Xnor,
    Or,
    Nor,
    And,
    Nand,
}

pub(crate) struct Alu {
    config: AluConfig,
    pub setting: AluSettings,
}

impl Alu {
    pub(crate) fn new(setting: AluSettings) -> Self {
        let config = match setting {
            AluSettings::Add => AluConfig {
                invert_a: false,
                invert_b: false,
                carry_in: false,
                flood_carry: false,
                xor_to_or: false,
            },
            AluSettings::Sub => AluConfig {
                invert_a: false,
                invert_b: true,
                carry_in: true,
                flood_carry: false,
                xor_to_or: false,
            },
            AluSettings::Xor => AluConfig {
                invert_a: false,
                invert_b: true,
                carry_in: false,
                flood_carry: true,
                xor_to_or: false,
            },
            AluSettings::Xnor => AluConfig {
                invert_a: false,
                invert_b: false,
                carry_in: false,
                flood_carry: true,
                xor_to_or: false,
            },
            AluSettings::Or => AluConfig {
                invert_a: false,
                invert_b: false,
                carry_in: false,
                flood_carry: false,
                xor_to_or: true,
            },
            AluSettings::Nor => AluConfig {
                invert_a: false,
                invert_b: false,
                carry_in: false,
                flood_carry: true,
                xor_to_or: true,
            },
            AluSettings::And => AluConfig {
                invert_a: true,
                invert_b: true,
                carry_in: false,
                flood_carry: true,
                xor_to_or: true,
            },
            AluSettings::Nand => AluConfig {
                invert_a: true,
                invert_b: true,
                carry_in: false,
                flood_carry: false,
                xor_to_or: true,
            },
        };
        Alu { config, setting }
    }

    pub(crate) fn compute(&self, mut a: Bits<8>, mut b: Bits<8>) -> Bits<8> {
        const CARRY_LENGTH: usize = 9;
        let config = self.config;

        if config.invert_b {
            b.iter_mut().for_each(|bit| *bit = !*bit);
        }

        if config.invert_a {
            a.iter_mut().for_each(|bit| *bit = !*bit);
        }

        let mut carry = [Signal::Cancel; CARRY_LENGTH];
        let mut xor_or = [false; 8];
        let mut res = [false; 8];
        for i in 0..CARRY_LENGTH {
            if i == 0 {
                carry[i..CARRY_LENGTH].fill(match config.carry_in {
                    true => Signal::Generate,
                    false => Signal::Cancel,
                });
                continue;
            }
            let a_bit = a[i - 1];
            let b_bit = b[i - 1];

            if config.xor_to_or {
                xor_or[i - 1] = a_bit | b_bit;
            } else {
                xor_or[i - 1] = a_bit ^ b_bit;
                if a_bit && b_bit {
                    carry[i..CARRY_LENGTH].fill(Signal::Generate);
                } else if !a_bit && !b_bit {
                    carry[i..CARRY_LENGTH].fill(Signal::Cancel);
                }
            }
        }

        if config.flood_carry {
            carry[0..CARRY_LENGTH].fill(Signal::Generate);
        }

        for i in 0..(CARRY_LENGTH - 1) {
            match carry[i] {
                Signal::Cancel => {
                    res[i] = xor_or[i] ^ false;
                }
                Signal::Generate => {
                    res[i] = xor_or[i] ^ true;
                }
            }
        }
        Bits::from(res)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn bits_to_u8(bits: Bits<8>) -> u8 {
        bits.into()
    }

    macro_rules! bits_8 {
        ($num:expr) => {{
            assert!(
                $num <= 0xFF,
                "bits_8! only accepts values fitting in 8 bits"
            );
            Bits::<8>::from($num as u8)
        }};
    }

    macro_rules! assert_bits {
        ($expr:expr, $expected:expr) => {
            assert_eq!(format!("{:08b}", bits_to_u8($expr)), $expected);
        };
    }

    #[test]
    fn test_convert_bit() {
        let a = 8u8;
        let res = Bits::from(a);
        let expected = Bits::<8>::from([false, false, false, true, false, false, false, false]);
        assert_eq!(res, expected);
    }

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
}
