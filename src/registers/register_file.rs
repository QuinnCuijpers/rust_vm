use crate::bits::Bits;
use crate::registers::Register;
use crate::Immediate;

const REGISTER_BANK_SIZE: usize = 16; // Number of registers in each bank
const REGISTER_SIZE: usize = 8; // Size of each register in bits

type DataRegister = Bits<REGISTER_SIZE>;
pub type RegisterBank = [DataRegister; REGISTER_BANK_SIZE];
type RegisterIndex = Bits<4>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisterFile {
    pub register_banks: [RegisterBank; 2], // Two sets of 16 registers, for simulated dual read
    enabled: bool,
    write_buffer: Option<(RegisterIndex, Immediate)>,
    read_addresses: [RegisterIndex; 2],
    pub read_outputs: [DataRegister; 2],
}

impl Default for RegisterFile {
    fn default() -> Self {
        RegisterFile {
            register_banks: [[Bits::from(0u8); REGISTER_BANK_SIZE]; 2],
            enabled: true,
            write_buffer: None,
            read_addresses: [Bits::default(); 2],
            read_outputs: [Bits::from(0u8); 2],
        }
    }
}

impl Register for RegisterFile {
    type WriteInformation = (RegisterIndex, Immediate);

    fn enable(&mut self, enabled: bool) {
        if !enabled {
            self.enabled = false;
            self.read_outputs = [Bits::from(0u8); 2];
            self.write_buffer = None;
        } else {
            self.enabled = true;
            for (r, index) in self.read_addresses.iter().enumerate() {
                if self.is_valid_index(*index) {
                    self.read_outputs[r] = self.register_banks[r][index.to_usize()];
                }
            }
        }
    }

    fn clock(&mut self) {
        if self.enabled {
            let Some((index, value)) = self.write_buffer.take() else {
                return; // No write scheduled
            };
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

    fn schedule_write(&mut self, write_info: Self::WriteInformation) {
        let (index, value) = write_info;
        if self.is_valid_index(index) && index != Bits::from(0u8).resize::<4>() {
            self.write_buffer = Some((index, value));
        }
    }
}

impl RegisterFile {
    pub fn new(rb: RegisterBank) -> Self {
        RegisterFile {
            register_banks: [rb; 2],
            enabled: true,
            ..Default::default()
        }
    }
    #[inline]
    fn is_valid_index(&self, index: RegisterIndex) -> bool {
        let index = index.to_usize();
        index < self.register_banks[0].len()
    }

    pub(crate) fn set_read_addresses(&mut self, indexes: [RegisterIndex; 2]) {
        if self.enabled {
            for (r, &index) in indexes.iter().enumerate() {
                if self.is_valid_index(index) {
                    self.read_addresses[r] = index;
                    self.read_outputs[r] = self.register_banks[r][index.to_usize()];
                }
            }
        }
    }

    pub fn display(&self) {
        for (i, bank) in self.register_banks.iter().enumerate() {
            print!("Register Bank {i}: ");
            for reg in bank.iter() {
                print!("{} ", reg.to_usize());
            }
            println!();
        }
    }
}
