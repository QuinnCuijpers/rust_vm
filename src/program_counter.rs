use crate::bits::Bits;

pub(crate) type Address = Bits<10>;
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PC {
    pub(crate) value: Address,
}

impl PC {
    pub(crate) fn clock(&mut self) -> Address {
        self.value
    }
}
