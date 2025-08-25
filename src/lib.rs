#![allow(dead_code)]
#![deny(clippy::all)]

mod alu;
pub mod bits;
mod control_rom;
mod error;
mod instruction_memory;
pub mod io_devices;
mod parser;
mod program_counter;
pub mod registers;
mod vm;

pub(crate) type ProgramInstruction = Bits<16>;
pub(crate) type Program = Vec<ProgramInstruction>;
pub(crate) type OpCode = Bits<4>;
pub(crate) type Immediate = Bits<8>;
pub(crate) type Address = Bits<10>;
pub(crate) type Condition = Bits<2>;
pub(crate) type MemoryAddress = Bits<8>; // 8-bit address for 256 bytes of memory
pub const OPCODE_HLT: Bits<4> = Bits {
    bit_array: [true, false, false, false],
};

pub use crate::bits::Bits;
pub use crate::bits::BitsParseError;
pub use crate::parser::error::ParserError;
pub use crate::vm::VM;

type Error = crate::error::VmError;
pub type Result<T> = std::result::Result<T, Error>;
