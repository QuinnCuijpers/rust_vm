use crate::BitsParseError;
use crate::ParserError;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum VmError {
    Parser(ParserError),
    Bits(BitsParseError),
    Io(io::Error),
    InstructionMemoryOverflow,
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmError::Parser(e) => write!(f, "Parser error: {e}"),
            VmError::Bits(e) => write!(f, "Bits error: {e}"),
            VmError::Io(e) => write!(f, "IO error: {e}"),
            VmError::InstructionMemoryOverflow => write!(f, "Instruction memory overflow. This error occurs when trying to load more instructions than the instruction memory can hold."),
        }
    }
}

impl std::error::Error for VmError {}

impl From<ParserError> for VmError {
    fn from(e: ParserError) -> Self {
        VmError::Parser(e)
    }
}
impl From<BitsParseError> for VmError {
    fn from(e: BitsParseError) -> Self {
        VmError::Bits(e)
    }
}
impl From<io::Error> for VmError {
    fn from(e: io::Error) -> Self {
        VmError::Io(e)
    }
}
