#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BitsParseError {
    Length { expected: usize, found: usize },
    Character { character: char },
    Number { source: std::num::ParseIntError },
    OutOfBounds { value: usize, max: usize },
}

impl std::fmt::Display for BitsParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BitsParseError::Length { expected, found } => {
                write!(f, "Invalid length: expected {expected}, found {found}")
            }
            BitsParseError::Character { character } => {
                write!(f, "Invalid character: '{character}'")
            }
            BitsParseError::Number { source } => write!(f, "Invalid number: {source}"),
            BitsParseError::OutOfBounds { value, max } => {
                write!(f, "Value {value} is out of bounds (max: {max})")
            }
        }
    }
}

impl std::error::Error for BitsParseError {}
