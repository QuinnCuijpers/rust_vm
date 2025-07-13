use crate::bits::Bits;

const REGISTER_BANK_SIZE: usize = 16;
const REGISTER_SIZE: usize = 8; // Number of registers in each bank

type Register = Bits<REGISTER_SIZE>; // Assuming 8-bit registers
type RegisterBank = [Register; REGISTER_BANK_SIZE];

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct RegisterFile {
    register_banks: [RegisterBank; 2], // Two sets of 16 registers, for simulated dual read
    enabled: bool,
    write_buffer: Vec<(usize, Register)>,
    read_addresses: [usize; 2],
    read_outputs: [Register; 2],
}

impl RegisterFile {
    #[inline]
    fn is_valid_index(&self, index: usize) -> bool {
        index < self.register_banks[0].len() && index != 0
    }

    fn set_read_addresses(&mut self, indexes: [usize; 2]) {
        if self.enabled {
            for (r, &index) in indexes.iter().enumerate() {
                if self.is_valid_index(index) {
                    self.read_addresses[r] = index;
                    self.read_outputs[r] = self.register_banks[r][index];
                }
            }
        }
    }

    fn schedule_write(&mut self, index: usize, value: Register) {
        if self.is_valid_index(index) {
            self.write_buffer.push((index, value));
        }
    }

    fn disable(&mut self) {
        self.enabled = false;
        self.read_outputs = [Bits::from(0u8); 2];
    }

    fn enable(&mut self) {
        self.enabled = true;
        for (r, index) in self.read_addresses.iter().enumerate() {
            if self.is_valid_index(*index) {
                self.read_outputs[r] = self.register_banks[r][*index];
            }
        }
    }

    fn clock(&mut self) {
        if self.enabled {
            let writes: Vec<_> = self.write_buffer.drain(..).collect();
            for (index, value) in writes {
                if self.is_valid_index(index) {
                    for r in 0..self.register_banks.len() {
                        // for simulated dual read, write to both sets
                        self.register_banks[r][index] = value;
                        if index == self.read_addresses[r] {
                            self.read_outputs[r] = value; // Update read values if the written index is being read
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests;
