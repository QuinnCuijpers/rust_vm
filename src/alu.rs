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

    pub(crate) fn compute<const N: usize>(&self, mut a: Bits<N>, mut b: Bits<N>) -> Bits<N> {
        let config = self.config;

        if config.is_rshift {
            let mut res = [false; N];
            for i in 0..(N - 1) {
                res[i] = a[i + 1];
            }
            res[N - 1] = false;
            return Bits::from(res);
        }

        if config.invert_b {
            b.iter_mut().for_each(|bit| *bit = !*bit);
        }

        if config.invert_a {
            a.iter_mut().for_each(|bit| *bit = !*bit);
        }

        let mut carry = vec![Signal::Cancel; N + 1];
        let mut xor_or = [false; N];
        let mut res = [false; N];
        for i in 0..(N + 1) {
            if i == 0 {
                carry[i..(N + 1)].fill(match config.carry_in {
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
                    carry[i..(N + 1)].fill(Signal::Generate);
                } else if !a_bit && !b_bit {
                    carry[i..(N + 1)].fill(Signal::Cancel);
                }
            }
        }

        if config.flood_carry {
            carry[0..(N + 1)].fill(Signal::Generate);
        }

        for i in 0..N {
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
