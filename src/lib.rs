#![allow(dead_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::redundant_test_prefix)]

mod alu;
mod bits;
mod control_rom;
mod error;
mod instruction_memory;
mod parser;
mod program_counter;
mod registers;
mod vm;

pub(crate) type ProgramInstruction = Bits<16>;
pub(crate) type Program = Vec<ProgramInstruction>;
pub(crate) type OpCode = Bits<4>;
pub(crate) type Immediate = Bits<8>;
pub(crate) type Address = Bits<10>;
pub(crate) type Condition = Bits<2>;

use crate::bits::Bits;
pub use crate::bits::BitsParseError;
pub use crate::parser::error::ParserError;
pub use crate::vm::VM;

type Error = crate::error::VmError;
pub type Result<T> = std::result::Result<T, Error>;
