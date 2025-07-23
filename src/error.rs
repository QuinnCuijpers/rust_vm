use crate::BitsParseError;
use crate::ParserError;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum VmError {
    Parser(ParserError),
    Bits(BitsParseError),
    Io(io::Error),
    NumberParse(std::num::ParseIntError),
    InstructionMemoryOverflow,
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmError::Parser(e) => write!(f, "Parser error: {e}"),
            VmError::Bits(e) => write!(f, "Bits error: {e}"),
            VmError::Io(e) => write!(f, "IO error: {e}"),
            VmError::NumberParse(e) => write!(f, "Number parse error: {e}"),
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
impl From<std::num::ParseIntError> for VmError {
    fn from(e: std::num::ParseIntError) -> Self {
        VmError::NumberParse(e)
    }
}

impl PartialEq for VmError {
    fn eq(&self, other: &Self) -> bool {
        use VmError::*;
        match (self, other) {
            (Parser(a), Parser(b)) => a == b,
            (Bits(a), Bits(b)) => a == b,
            (InstructionMemoryOverflow, InstructionMemoryOverflow) => true,
            // Io and NumberParse are not comparable
            _ => false,
        }
    }
}
