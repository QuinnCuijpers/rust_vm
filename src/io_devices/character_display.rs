// Character Display	247	Store Only	Write Char	Character	Write character to buffer
// 	248	Store Only	Buffer Chars		Push character buffer
// 	249	Store Only	Clear Chars Buffer		Clear character buffer

use crate::{bits::Bits, io_devices::Device};

const BUFFER_SIZE: usize = 10; // Maximum number of characters in the buffer

const CHARACTERS: &str = " abcdefghijklmnopqrstuvwxyz.!?";

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CharacterDisplay {
    pub(crate) buffer: String,
    pub active: String,
}

impl Default for CharacterDisplay {
    fn default() -> Self {
        CharacterDisplay {
            buffer: String::with_capacity(BUFFER_SIZE),
            active: String::with_capacity(BUFFER_SIZE),
        }
    }
}

impl CharacterDisplay {
    pub fn new() -> Self {
        CharacterDisplay {
            buffer: String::with_capacity(BUFFER_SIZE),
            active: String::with_capacity(BUFFER_SIZE),
        }
    }

    pub fn display(&self) {
        println!("Character Display: {}", self.active);
    }

    pub fn buffer_display(&self) {
        println!("Character Buffer: {}", self.buffer);
    }
}

impl Device for CharacterDisplay {
    fn on_read(&mut self, _addr: crate::MemoryAddress) -> Bits<8> {
        Bits::default() // Character display does not support reading, returns 0
    }

    fn on_write(&mut self, addr: crate::MemoryAddress, value: Bits<8>) {
        match addr.to_usize() {
            247 => {
                if self.buffer.len() < BUFFER_SIZE {
                    if let Some(c) = CHARACTERS.chars().nth(value.to_usize()) {
                        self.buffer.push(c);
                    }
                }
            }
            248 => {
                self.active.push_str(&self.buffer);
            }
            249 => {
                self.buffer.clear(); // Clear character buffer
            }
            _ => {}
        }
    }
}
