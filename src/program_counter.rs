use crate::Address;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PC {
    pub(crate) value: Address,
}

impl PC {
    pub(crate) fn clock(&mut self) -> Address {
        self.value
    }
}
