use crate::{bits::Bits, io_devices::Device, MemoryAddress};

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RNG {
    seed: Bits<8>,
    state: Bits<8>,
}

impl Default for RNG {
    fn default() -> Self {
        let mut seed = Bits::from(rand::random::<u8>());
        while seed == Bits::from(0u8) {
            seed = Bits::from(rand::random::<u8>());
        }
        RNG { seed, state: seed }
    }
}

impl RNG {
    pub fn new(seed: Bits<8>) -> Self {
        assert!(seed > Bits::from(0u8), "Seed must be greater than 0");
        Self { seed, state: seed }
    }

    pub fn generate_next(&mut self) -> Bits<8> {
        // Linear shift generator algorithm
        let mut lfsr = self.state;
        let bit = ((lfsr >> Bits::from(7u8))
            ^ (lfsr >> Bits::from(5u8))
            ^ (lfsr >> Bits::from(4u8))
            ^ (lfsr >> Bits::from(3u8)))
            & Bits::from(1u8);
        lfsr = (lfsr << Bits::from(1u8)) | bit;
        self.state = lfsr;
        self.state
    }
}

impl Device for RNG {
    fn on_read(&mut self, _addr: MemoryAddress) -> Bits<8> {
        self.generate_next()
    }

    fn on_write(&mut self, _addr: MemoryAddress, _value: Bits<8>) {} // RNG does not support writing
}
