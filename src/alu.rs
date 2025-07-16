use crate::bits::Bits;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Signal {
    Cancel,
    Generate,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(crate) struct AluConfig {
    invert_a: bool,
    invert_b: bool,
    carry_in: bool,
    flood_carry: bool,
    xor_to_or: bool,
    is_rshift: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Eq)]
pub(crate) enum AluSettings {
    #[default]
    Add,
    Sub,
    Xor,
    Xnor,
    Or,
    Nor,
    And,
    Nand,
    Implies,
    Nimplies,
    Rshift,
}

impl AluSettings {
    pub(crate) fn config(&self, setting: AluSettings) -> AluConfig {
        match setting {
            AluSettings::Add => AluConfig {
                invert_a: false,
                invert_b: false,
                carry_in: false,
                flood_carry: false,
                xor_to_or: false,
                is_rshift: false,
            },
            AluSettings::Sub => AluConfig {
                invert_a: false,
                invert_b: true,
                carry_in: true,
                flood_carry: false,
                xor_to_or: false,
                is_rshift: false,
            },
            AluSettings::Xor => AluConfig {
                invert_a: false,
                invert_b: true,
                carry_in: false,
                flood_carry: true,
                xor_to_or: false,
                is_rshift: false,
            },
            AluSettings::Xnor => AluConfig {
                invert_a: false,
                invert_b: false,
                carry_in: false,
                flood_carry: true,
                xor_to_or: false,
                is_rshift: false,
            },
            AluSettings::Or => AluConfig {
                invert_a: false,
                invert_b: false,
                carry_in: false,
                flood_carry: false,
                xor_to_or: true,
                is_rshift: false,
            },
            AluSettings::Nor => AluConfig {
                invert_a: false,
                invert_b: false,
                carry_in: false,
                flood_carry: true,
                xor_to_or: true,
                is_rshift: false,
            },
            AluSettings::And => AluConfig {
                invert_a: true,
                invert_b: true,
                carry_in: false,
                flood_carry: true,
                xor_to_or: true,
                is_rshift: false,
            },
            AluSettings::Nand => AluConfig {
                invert_a: true,
                invert_b: true,
                carry_in: false,
                flood_carry: false,
                xor_to_or: true,
                is_rshift: false,
            },
            AluSettings::Implies => AluConfig {
                invert_a: true,
                invert_b: false,
                carry_in: false,
                flood_carry: false,
                xor_to_or: true,
                is_rshift: false,
            },
            AluSettings::Nimplies => AluConfig {
                invert_a: true,
                invert_b: false,
                carry_in: false,
                flood_carry: true,
                xor_to_or: true,
                is_rshift: false,
            },
            AluSettings::Rshift => AluConfig {
                invert_a: false,
                invert_b: false,
                carry_in: false,
                flood_carry: false,
                xor_to_or: false,
                is_rshift: true,
            },
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct Alu {
    config: AluConfig,
    pub setting: AluSettings,
}

impl Alu {
    pub(crate) fn new(setting: AluSettings) -> Self {
        let config = setting.config(setting);
        Self { config, setting }
    }

    pub(crate) fn compute(&self, mut a: Bits<8>, mut b: Bits<8>) -> Bits<8> {
        const CARRY_LENGTH: usize = 9;
        let config = self.config;

        if config.is_rshift {
            let mut res = [false; 8];
            for i in 0..7 {
                res[i] = a[i + 1]; // right shift returns a shifted right by 1
            }
            res[7] = false;
            return Bits::from(res);
        }

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

    pub(crate) fn set_setting(&mut self, setting: AluSettings) {
        self.setting = setting;
        self.config = setting.config(setting);
    }
}

#[cfg(test)]
mod tests;
