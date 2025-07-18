// TODO: Add more specific error types as needed
#[derive(Debug)]
pub enum ParserError {
    FileNotFound(String),
    InvalidInstruction(String),
    MissingOperand(String),
    UndefinedLabel(String),
    TooManyOperands(String),
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::FileNotFound(file) => write!(f, "File not found: {file}"),
            ParserError::InvalidInstruction(instr) => write!(f, "Invalid instruction: {instr}"),
            ParserError::MissingOperand(line) => write!(f, "Missing operand in line: {line}"),
            ParserError::UndefinedLabel(label) => write!(f, "Undefined label: {label}"),
            ParserError::TooManyOperands(line) => write!(f, "Too many operands in line: {line}"),
        }
    }
}

impl std::error::Error for ParserError {}
