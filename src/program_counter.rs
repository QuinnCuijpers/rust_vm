use crate::Address;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PC {
    pub value: Address,
}

impl PC {
    pub fn clock(&mut self) -> Address {
        self.value
    }
}
