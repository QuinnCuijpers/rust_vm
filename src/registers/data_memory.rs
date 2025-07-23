use crate::{bits::Bits, registers::Register, MemoryAddress};

const MEMORY_SIZE: usize = 256; // Size of the data memory in bytes

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum MemoryState {
    #[default]
    Disabled,
    Read,
    Write,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataMemory {
    // TODO: find appropriate type alias
    pub memory: [Bits<8>; MEMORY_SIZE], // 256 bytes of memory
    pub enabled: bool,
    state: MemoryState,
    write_buffer: Option<(MemoryAddress, Bits<8>)>,
}

impl Default for DataMemory {
    fn default() -> Self {
        DataMemory {
            memory: [Bits::from(0u8); MEMORY_SIZE],
            enabled: true,
            state: MemoryState::Read,
            write_buffer: None,
        }
    }
}

impl Register for DataMemory {
    type WriteInformation = (MemoryAddress, Bits<8>);

    fn enable(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.write_buffer = None;
        }
    }

    fn clock(&mut self) {
        if self.enabled {
            let Some((address, data)) = self.write_buffer.take() else {
                return; // No write scheduled
            };
            self.memory[address.to_usize()] = data;
        }
    }

    fn schedule_write(&mut self, write_info: Self::WriteInformation) {
        if self.enabled && self.state == MemoryState::Write {
            self.write_buffer = Some(write_info);
        }
    }
}

impl DataMemory {
    pub fn read(&self, address: MemoryAddress) -> Bits<8> {
        if self.enabled && self.state == MemoryState::Read {
            self.memory[address.to_usize()]
        } else {
            Bits::from(0u8) // Return zero if not enabled or not in read state
        }
    }

    pub(crate) fn set_state(&mut self, state: MemoryState) {
        self.state = state;
    }
}
