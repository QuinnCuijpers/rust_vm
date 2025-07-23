// Screen:
//     240  Store Only   Pixel X            Bottom 5 bits are X coordinate
//     241  Store Only   Pixel Y            Bottom 5 bits are Y coordinate
//     242  Store Only   Draw Pixel         Draw pixel at (Pixel X, Pixel Y) to buffer
//     243  Store Only   Clear Pixel        Clear pixel at (Pixel X, Pixel Y) to buffer
//     244  Load Only    Load Pixel         Pixel Data (0 or 1)    Load Pixel at (Pixel X, Pixel Y)
//     245  Store Only   Buffer Screen      Push screen buffer
//     246  Store Only   Clear Screen Buffer Clear screen buffer

use crate::{bits::Bits, io_devices::Device};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct Screen {
    pub current_x: usize,
    pub current_y: usize,
    pub buffer: [[bool; 32]; 32],
    pub active: [[bool; 32]; 32],
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            buffer: [[false; 32]; 32],
            active: [[false; 32]; 32],
            current_x: 0,
            current_y: 0,
        }
    }
}

impl Device for Screen {
    fn on_read(&mut self, addr: crate::MemoryAddress) -> Bits<8> {
        assert_eq!(
            addr.to_usize(),
            244,
            "Load Pixel should only be called with address 244"
        );
        // Return the pixel value at the current coordinates
        let x = self.current_x & 0x1F; // Ensure X is within bounds
        let y = self.current_y & 0x1F; // Ensure Y is within bounds
        Bits::from(self.active[y][x] as u8) // Convert bool to Bits<8>
    }

    fn on_write(&mut self, addr: crate::MemoryAddress, value: Bits<8>) {
        let bot_5: Bits<5> = value.slice(0);
        let val = bot_5.to_usize();
        if val < 32 {
            match addr.to_usize() {
                240 => self.current_x = val,                               // set Pixel X
                241 => self.current_y = val,                               // set Pixel Y
                242 => self.buffer[self.current_y][self.current_x] = true, // Draw pixel
                243 => self.buffer[self.current_y][self.current_x] = false, // Clear pixel
                245 => self.active = self.buffer,                          // Buffer screen
                246 => self.buffer = [[false; 32]; 32],                    // Clear screen buffer
                _ => {}
            }
        }
    }
}
