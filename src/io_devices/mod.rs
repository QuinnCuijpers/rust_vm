use crate::{bits::Bits, MemoryAddress};

pub mod character_display;
pub mod number_display;
pub mod rng;
pub mod screen;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct IoDevices {
    pub character_display: character_display::CharacterDisplay,
    pub number_display: number_display::NumberDisplay,
    pub rng: rng::RNG,
    pub screen: screen::Screen,
}

impl Device for IoDevices {
    fn on_read(&mut self, addr: MemoryAddress) -> Bits<8> {
        match addr.to_usize() {
            244 => self.screen.on_read(addr), // Load Pixel
            _ => Bits::default(),             // Default for other addresses
        }
    }

    fn on_write(&mut self, addr: MemoryAddress, value: Bits<8>) {
        match addr.to_usize() {
            240..=246 => self.screen.on_write(addr, value), // Screen operations
            247..=249 => self.character_display.on_write(addr, value), // Character Display operations
            250..=253 => self.number_display.on_write(addr, value),    // Number Display operations
            254 => self.rng.on_write(addr, value),                     // RNG operations
            _ => {} // No operation for other addresses
        }
    }
}

pub(crate) trait Device {
    fn on_read(&mut self, addr: MemoryAddress) -> Bits<8>;
    fn on_write(&mut self, addr: MemoryAddress, value: Bits<8>);
}

#[cfg(test)]
mod test {
    use super::number_display::NumberDisplay;
    use super::rng::RNG;
    use crate::bits::Bits;
    use crate::MemoryAddress;

    #[test]
    fn next_rng() {
        let seed = Bits::from(0xACu8);
        let mut rng = RNG::new(seed);
        for i in 0..10 {
            let next = rng.generate_next();
            assert!(
                next > Bits::from(0u8),
                "RNG output should be greater than 0"
            );
            println!("RNG output {i}: {}", next.to_usize());
        }
    }
    #[test]
    fn number_display() {
        use super::Device;
        let mut display = NumberDisplay {
            display: Bits::from(0u8),
            state: super::number_display::DisplayState::default(),
            active: true,
        };

        display.on_write(MemoryAddress::from(250u8), Bits::from(42u8));
        assert_eq!(display.display, Bits::from(42));

        display.on_write(MemoryAddress::from(251u8), Bits::from(0u8));
        assert!(!display.active);

        display.on_write(MemoryAddress::from(252u8), Bits::from(0u8));
        assert_eq!(
            display.state,
            super::number_display::DisplayState::SignedMode
        );

        display.on_write(MemoryAddress::from(253u8), Bits::from(0u8));
        assert_eq!(
            display.state,
            super::number_display::DisplayState::UnsignedMode
        );
    }

    #[test]
    fn character_display() {
        use super::Device;
        let mut display = super::character_display::CharacterDisplay::new();

        display.on_write(MemoryAddress::from(247u8), Bits::from(1u8)); // 'a'
        assert_eq!(display.buffer, "a");

        display.on_write(MemoryAddress::from(248u8), Bits::default());
        assert_eq!(display.active, "a");

        display.on_write(MemoryAddress::from(249u8), Bits::default());
        assert!(display.buffer.is_empty());
    }
}
