use crate::bits::Bits;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PC {
    pub(crate) value: Bits<10>,
}

impl PC {
    pub(crate) fn clock(&mut self) -> Bits<10> {
        let current_value = self.value;
        self.increment();
        current_value
    }

    fn increment(&mut self) {
        self.value += Bits::from(1u16).resize::<10>();
    }
}
