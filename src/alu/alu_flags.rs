use crate::bits::Bits;

pub(crate) type Condition = Bits<2>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct AluFlags {
    pub(crate) zero: bool,
    pub(crate) carry: bool,
}

impl AluFlags {
    pub(crate) fn set_zero(&mut self, value: bool) {
        self.zero = value;
    }

    pub(crate) fn set_carry(&mut self, value: bool) {
        self.carry = value;
    }

    pub(crate) fn cond_true(&self, condition: Condition) -> bool {
        match condition.to_string().as_str() {
            "00" => self.zero,
            "01" => !self.zero,
            "10" => self.carry,
            "11" => !self.carry,
            _ => unreachable!(),
        }
    }
}
