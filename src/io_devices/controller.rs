use crate::{bits::Bits, io_devices::Device, MemoryAddress};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct Controller {
    pub left: bool,
    pub down: bool,
    pub right: bool,
    pub up: bool,
    pub b: bool,
    pub a: bool,
    pub select: bool,
    pub start: bool,
    pub value: Bits<8>,
}

impl Device for Controller {
    fn on_read(&mut self, _address: MemoryAddress) -> Bits<8> {
        self.value
    }

    fn on_write(&mut self, _address: MemoryAddress, _value: Bits<8>) {
        // Controller is read-only; ignore writes
    }
}

impl Controller {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_a(&mut self, pressed: bool) {
        self.a = pressed;
        if pressed {
            self.value |= Bits::from(0b0010_0000).resize();
        } else {
            self.value &= Bits::from(0b1101_1111).resize();
        }
    }

    pub fn set_b(&mut self, pressed: bool) {
        self.b = pressed;
        if pressed {
            self.value |= Bits::from(0b0001_0000).resize();
        } else {
            self.value &= Bits::from(0b1110_1111).resize();
        }
    }

    pub fn set_up(&mut self, pressed: bool) {
        self.up = pressed;
        if pressed {
            self.value |= Bits::from(0b0000_1000).resize();
        } else {
            self.value &= Bits::from(0b1111_0111).resize();
        }
    }

    pub fn set_down(&mut self, pressed: bool) {
        self.down = pressed;
        if pressed {
            self.value |= Bits::from(0b0000_0010).resize();
        } else {
            self.value &= Bits::from(0b1111_1101).resize();
        }
    }

    pub fn set_left(&mut self, pressed: bool) {
        self.left = pressed;

        if pressed {
            self.value |= Bits::from(0b0000_0001).resize();
        } else {
            self.value &= Bits::from(0b1111_1110).resize();
        }
    }

    pub fn set_right(&mut self, pressed: bool) {
        self.right = pressed;
        if pressed {
            self.value |= Bits::from(0b0000_0100).resize();
        } else {
            self.value &= Bits::from(0b1111_1011).resize();
        }
    }

    pub fn set_select(&mut self, pressed: bool) {
        self.select = pressed;
        if pressed {
            self.value |= Bits::from(0b0100_0000).resize();
        } else {
            self.value &= Bits::from(0b1011_1111).resize();
        }
    }

    pub fn set_start(&mut self, pressed: bool) {
        self.start = pressed;
        if pressed {
            self.value |= Bits::from(0b1000_0000).resize();
        } else {
            self.value &= Bits::from(0b0111_1111).resize();
        }
    }
}
