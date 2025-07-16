use std::path::Path;
use std::{fs, str::FromStr};

use crate::bits::Bits;

#[derive(Debug)]
pub enum ParserError {
    FileNotFound(String),
    InvalidInstruction(String),
    MissingOperand(String),
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::FileNotFound(file) => write!(f, "File not found: {}", file),
            ParserError::InvalidInstruction(instr) => write!(f, "Invalid instruction: {}", instr),
            ParserError::MissingOperand(line) => write!(f, "Missing operand in line: {}", line),
        }
    }
}

impl std::error::Error for ParserError {}

// TODO: add proper error
fn parse_register_string(s: &str) -> Result<Bits<4>, Box<dyn std::error::Error>> {
    if s.len() < 2 || !s.starts_with('r') {
        return Err(Box::new(ParserError::InvalidInstruction(s.to_string())));
    }
    Ok(Bits::from_str(&s[1..])?)
}

pub(crate) fn parse_program(file_path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    let mut output_lines = vec![];

    let path = file_path.as_ref();
    let content = std::fs::read_to_string(path)
        .map_err(|_| ParserError::FileNotFound(path.display().to_string()))?;
    let lines = content.lines();

    for line in lines {
        let mut out = vec![];
        let mut splitted = line.split_whitespace();
        let instruction = match splitted.next() {
            Some(instr) => instr,
            None => continue, // Skip empty lines
        };
        let mut operands = splitted;
        match instruction {
            "NOP" => {
                let instruction_bits = parse_instruction(instruction)?;
                out.push(instruction_bits.to_string());
                out.push("0000 0000 0000".to_string()); // NOP has no operands and is parsed as all zeros
                if operands.next().is_some() {
                    return Err(Box::new(ParserError::MissingOperand(line.to_string())));
                }
            }
            "ADD" | "SUB" | "AND" | "NOR" | "XOR" => {
                let a = match operands.next() {
                    Some(op) => op,
                    None => return Err(Box::new(ParserError::MissingOperand(line.to_string()))),
                };
                let b = match operands.next() {
                    Some(op) => op,
                    None => return Err(Box::new(ParserError::MissingOperand(line.to_string()))),
                };
                let write = match operands.next() {
                    Some(op) => op,
                    None => return Err(Box::new(ParserError::MissingOperand(line.to_string()))),
                };
                if operands.next().is_some() {
                    return Err(Box::new(ParserError::MissingOperand(line.to_string())));
                }
                out.push(parse_instruction(instruction)?.to_string());
                out.push(parse_register_string(a)?.to_string());
                out.push(parse_register_string(b)?.to_string());
                out.push(parse_register_string(write)?.to_string());
            }
            "LDI" => {
                let a = match operands.next() {
                    Some(op) => op,
                    None => return Err(Box::new(ParserError::MissingOperand(line.to_string()))),
                };
                let values = match operands.next() {
                    Some(op) => Bits::<8>::from_str(op)?.split_into_chunks::<4>(),
                    None => return Err(Box::new(ParserError::MissingOperand(line.to_string()))),
                };
                if operands.next().is_some() {
                    return Err(Box::new(ParserError::MissingOperand(line.to_string())));
                }
                out.push(parse_instruction(instruction)?.to_string());
                out.push(parse_register_string(a)?.to_string());
                for value in values {
                    out.push(value.to_string());
                }
            }
            "RSH" => {
                let a = match operands.next() {
                    Some(op) => op,
                    None => return Err(Box::new(ParserError::MissingOperand(line.to_string()))),
                };
                let write = match operands.next() {
                    Some(op) => op,
                    None => return Err(Box::new(ParserError::MissingOperand(line.to_string()))),
                };
                if operands.next().is_some() {
                    return Err(Box::new(ParserError::MissingOperand(line.to_string())));
                }
                out.push(parse_instruction(instruction)?.to_string());
                out.push(parse_register_string(a)?.to_string());
                out.push("0000".to_string()); // RSH has no second operand
                out.push(parse_register_string(write)?.to_string());
            }
            _ => {
                return Err(Box::new(ParserError::InvalidInstruction(
                    instruction.to_string(),
                )))
            }
        }
        output_lines.push(out.join(" "));
    }

    use std::io::Write;
    let mut output_file = fs::File::create(path.with_extension("mc"))?;
    for l in output_lines {
        writeln!(output_file, "{}", l)?;
    }

    Ok(())
}

fn parse_instruction(instruction: &str) -> Result<Bits<4>, Box<dyn std::error::Error>> {
    let instruction_bits: Bits<4> = match instruction {
        "NOP" => Bits::from_str("0000")?,
        "ADD" => Bits::from_str("0010")?,
        "SUB" => Bits::from_str("0011")?,
        "AND" => Bits::from_str("0100")?,
        "NOR" => Bits::from_str("0101")?,
        "XOR" => Bits::from_str("0110")?,
        "RSH" => Bits::from_str("0111")?,
        "LDI" => Bits::from_str("1000")?,
        &_ => {
            return Err(Box::new(ParserError::InvalidInstruction(
                instruction.to_string(),
            )))
        }
    };
    Ok(instruction_bits)
}

#[cfg(test)]
mod tests;
