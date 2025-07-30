pub(crate) mod alu_flags;
pub(crate) mod alu_settings;

pub(crate) use alu_settings::AluSettings;

use crate::alu::alu_flags::AluFlags;
use crate::alu::alu_settings::AluConfig;
use crate::bits::Bits;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Signal {
    Cancel,
    Generate,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Alu {
    config: AluConfig,
    pub setting: AluSettings,
    pub flags: AluFlags,
    pub set_flags: bool,
}

impl Alu {
    pub(crate) fn new(setting: AluSettings) -> Self {
        let config = setting.config(setting);
        Self {
            config,
            setting,
            ..Default::default()
        }
    }

    pub(crate) fn compute<const N: usize>(&mut self, mut a: Bits<N>, mut b: Bits<N>) -> Bits<N> {
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
        let res = Bits::from(res);
        if self.set_flags {
            self.flags.set_zero(res == Bits::from(0u8));
            self.flags.set_carry(carry[N] == Signal::Generate);
        }

        res
    }

    pub(crate) fn set_setting(&mut self, setting: AluSettings) {
        self.setting = setting;
        self.config = setting.config(setting);
    }
}

#[cfg(test)]
mod tests;
