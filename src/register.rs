use crate::bits::Bits;

// TODO: improve magic numbers in file
const REGISTER_BANK_SIZE: usize = 16;
const REGISTER_SIZE: usize = 8; // Number of registers in each bank

pub type Register = Bits<REGISTER_SIZE>; // Assuming 8-bit registers
pub type RegisterBank = [Register; REGISTER_BANK_SIZE];

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct RegisterFile {
    pub(crate) register_banks: [RegisterBank; 2], // Two sets of 16 registers, for simulated dual read
    enabled: bool,
    write_buffer: Vec<(Bits<4>, Bits<8>)>,
    read_addresses: [Bits<4>; 2],
    pub(crate) read_outputs: [Register; 2],
}

impl RegisterFile {
    pub(crate) fn new(rb: RegisterBank) -> Self {
        RegisterFile {
            register_banks: [rb; 2],
            enabled: true,
            ..Default::default()
        }
    }
    #[inline]
    fn is_valid_index(&self, index: Bits<4>) -> bool {
        let index = index.to_usize();
        index < self.register_banks[0].len() && index != 0
    }

    // TODO: return Result
    pub(crate) fn set_read_addresses(&mut self, indexes: [Bits<4>; 2]) {
        if self.enabled {
            for (r, &index) in indexes.iter().enumerate() {
                if self.is_valid_index(index) {
                    self.read_addresses[r] = index;
                    self.read_outputs[r] = self.register_banks[r][index.to_usize()];
                }
            }
        }
    }

    pub(crate) fn schedule_write(&mut self, index: Bits<4>, value: Bits<8>) {
        if self.is_valid_index(index) {
            self.write_buffer.push((index, value));
        }
    }

    fn disable(&mut self) {
        self.enabled = false;
        self.read_outputs = [Bits::from(0u8); 2];
    }

    pub(crate) fn enable(&mut self) {
        self.enabled = true;
        for (r, index) in self.read_addresses.iter().enumerate() {
            if self.is_valid_index(*index) {
                self.read_outputs[r] = self.register_banks[r][index.to_usize()];
            }
        }
    }

    pub(crate) fn clock(&mut self) {
        if self.enabled {
            let writes: Vec<_> = self.write_buffer.drain(..).collect();
            for (index, value) in writes {
                if self.is_valid_index(index) {
                    for r in 0..self.register_banks.len() {
                        // for simulated dual read, write to both sets
                        self.register_banks[r][index.to_usize()] = value.resize();
                        if index == self.read_addresses[r] {
                            self.read_outputs[r] = value.resize(); // Update read values if the written index is being read
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests;
