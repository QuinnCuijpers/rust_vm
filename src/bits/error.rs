#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BitsParseError {
    Length {
        expected: usize,
        found: usize,
        string: String,
    },
    Character {
        character: char,
    },
    Number {
        source: std::num::ParseIntError,
        num: String,
    },
    OutOfBounds {
        value: usize,
        max: usize,
    },
}

impl std::fmt::Display for BitsParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BitsParseError::Length {
                expected,
                found,
                string,
            } => {
                write!(
                    f,
                    "Invalid length: expected {expected}, found {found} (input: {string})"
                )
            }
            BitsParseError::Character { character } => {
                write!(f, "Invalid character: '{character}'")
            }
            BitsParseError::Number { source, num } => {
                write!(f, "Invalid number: {source} (input: {num})")
            }
            BitsParseError::OutOfBounds { value, max } => {
                write!(f, "Value {value} is out of bounds (max: {max})")
            }
        }
    }
}

impl std::error::Error for BitsParseError {}
