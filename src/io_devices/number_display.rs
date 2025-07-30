use crate::{bits::Bits, io_devices::Device};

// 250	Store Only	Show Number	Number	Show number display
// 251	Store Only	Clear Number		Clear number display
// 252	Store Only	Signed Mode		Interpret number as 2s comp [-128, 127]
// 253	Store Only	Unsigned Mode		Interpret number as unsigned int [0, 255]

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub enum DisplayState {
    SignedMode,
    #[default]
    UnsignedMode,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, Copy)]
pub struct NumberDisplay {
    pub display: Bits<8>,
    pub state: DisplayState,
    pub active: bool, // Indicates if the display is active
}

impl NumberDisplay {
    pub fn new() -> Self {
        Self {
            display: Bits::default(),
            state: DisplayState::UnsignedMode,
            active: true,
        }
    }

    pub fn display(&self) {
        println!("Number Display: {}", self.get_display_val());
    }

    pub fn get_display_val(&self) -> String {
        if !self.active {
            return "0".to_string();
        }
        match self.state {
            DisplayState::SignedMode => i8::from(self.display).to_string(),
            DisplayState::UnsignedMode => u8::from(self.display).to_string(),
        }
    }
}

impl Device for NumberDisplay {
    fn on_read(&mut self, _addr: crate::MemoryAddress) -> Bits<8> {
        Bits::default() // Number display does not support reading so returns 0
    }

    fn on_write(&mut self, addr: crate::MemoryAddress, value: Bits<8>) {
        match addr.to_usize() {
            250 => {
                self.active = true; // Activate number display
                self.display = value; // Store number
            }
            251 => {
                self.active = false; // Clear number display
            }
            252 => {
                self.state = DisplayState::SignedMode; // Set signed mode
            }
            253 => {
                self.state = DisplayState::UnsignedMode; // Set unsigned mode
            }
            _ => {}
        }
    }
}
