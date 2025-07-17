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
mod register;
mod utils;
mod vm;

pub use crate::bits::BitsParseError;
pub use crate::parser::ParserError;
pub use crate::vm::VM;

type Error = crate::error::VmError;
pub type Result<T> = std::result::Result<T, Error>;
