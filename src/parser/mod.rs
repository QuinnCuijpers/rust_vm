use std::fs;
use std::path::Path;
use std::str::FromStr;

use crate::bits::Bits;
use crate::program_counter::Address;
use crate::Result;

// TODO: split parser into multiple files for better organization
#[derive(Debug)]
pub enum ParserError {
    FileNotFound(String),
    InvalidInstruction(String),
    MissingOperand(String),
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::FileNotFound(file) => write!(f, "File not found: {file}"),
            ParserError::InvalidInstruction(instr) => write!(f, "Invalid instruction: {instr}"),
            ParserError::MissingOperand(line) => write!(f, "Missing operand in line: {line}"),
        }
    }
}

impl std::error::Error for ParserError {}

// TODO: add proper error
fn parse_register_string(s: &str) -> Result<Bits<4>> {
    if s.len() < 2 || !s.starts_with('r') {
        return Err(ParserError::InvalidInstruction(s.to_string()).into());
    }
    Ok(Bits::from_str(&s[1..])?)
}

#[allow(clippy::unwrap_used)]
pub(crate) fn parse_program(file_path: impl AsRef<Path>) -> Result<()> {
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
            "NOP" | "HLT" => {
                let instruction_bits = parse_instruction(instruction).unwrap();
                out.push(instruction_bits.to_string());
                out.push("0000 0000 0000".to_string());
                if operands.next().is_some() {
                    return Err(ParserError::MissingOperand(line.to_string()).into());
                }
            }
            "INC" | "DEC" => {
                let a = match operands.next() {
                    Some(op) => op,
                    None => return Err(ParserError::MissingOperand(line.to_string()).into()),
                };
                if instruction == "INC" {
                    out.push(parse_instruction("ADI").unwrap().to_string());
                    out.push(parse_register_string(a)?.to_string());
                    out.push(Bits::from(1u8).to_string());
                } else {
                    out.push(parse_instruction("ADI").unwrap().to_string());
                    out.push(parse_register_string(a)?.to_string());
                    out.push(Bits::from(255u8).to_string());
                }
                if operands.next().is_some() {
                    return Err(ParserError::MissingOperand(line.to_string()).into());
                }
            }
            "JMP" => {
                let a = match operands.next() {
                    Some(op) => op,
                    None => return Err(ParserError::MissingOperand(line.to_string()).into()),
                };
                if operands.next().is_some() {
                    return Err(ParserError::MissingOperand(line.to_string()).into());
                }
                out.push(parse_instruction("JMP").unwrap().to_string());
                out.push(Bits::<2>::from_str("00").unwrap().to_string());
                out.push(parse_address(a)?.to_string());
            }
            "CMP" => {
                let a = match operands.next() {
                    Some(op) => op,
                    None => return Err(ParserError::MissingOperand(line.to_string()).into()),
                };
                let b = match operands.next() {
                    Some(op) => op,
                    None => return Err(ParserError::MissingOperand(line.to_string()).into()),
                };
                if operands.next().is_some() {
                    return Err(ParserError::MissingOperand(line.to_string()).into());
                }
                // CMP rx ry -> SUB rx ry r0
                out.push(parse_instruction("SUB").unwrap().to_string());
                out.push(parse_register_string(a)?.to_string());
                out.push(parse_register_string(b)?.to_string());
                out.push("0000".to_string())
            }
            "LDI" | "ADI" => {
                let a = match operands.next() {
                    Some(op) => op,
                    None => return Err(ParserError::MissingOperand(line.to_string()).into()),
                };
                let values = match operands.next() {
                    Some(op) => Bits::<8>::from_str(op)?,
                    None => return Err(ParserError::MissingOperand(line.to_string()).into()),
                };
                if operands.next().is_some() {
                    return Err(ParserError::MissingOperand(line.to_string()).into());
                }
                out.push(parse_instruction(instruction).unwrap().to_string());
                out.push(parse_register_string(a)?.to_string());
                out.push(values.to_string());
            }
            "RSH" => {
                let a = match operands.next() {
                    Some(op) => op,
                    None => return Err(ParserError::MissingOperand(line.to_string()).into()),
                };
                let write = match operands.next() {
                    Some(op) => op,
                    None => return Err(ParserError::MissingOperand(line.to_string()).into()),
                };
                if operands.next().is_some() {
                    return Err(ParserError::MissingOperand(line.to_string()).into());
                }
                out.push(parse_instruction("RSH").unwrap().to_string());
                out.push(parse_register_string(a)?.to_string());
                out.push("0000".to_string());
                out.push(parse_register_string(write)?.to_string());
            }
            "ADD" | "SUB" | "AND" | "NOR" | "XOR" => {
                let a = match operands.next() {
                    Some(op) => op,
                    None => return Err(ParserError::MissingOperand(line.to_string()).into()),
                };
                let b = match operands.next() {
                    Some(op) => op,
                    None => return Err(ParserError::MissingOperand(line.to_string()).into()),
                };
                let write = match operands.next() {
                    Some(op) => op,
                    None => return Err(ParserError::MissingOperand(line.to_string()).into()),
                };
                if operands.next().is_some() {
                    return Err(ParserError::MissingOperand(line.to_string()).into());
                }
                out.push(parse_instruction(instruction).unwrap().to_string());
                out.push(parse_register_string(a)?.to_string());
                out.push(parse_register_string(b)?.to_string());
                out.push(parse_register_string(write)?.to_string());
            }
            "BRH" => {
                let a = match operands.next() {
                    Some(op) => op,
                    None => return Err(ParserError::MissingOperand(line.to_string()).into()),
                };
                let addr = match operands.next() {
                    Some(op) => op,
                    None => return Err(ParserError::MissingOperand(line.to_string()).into()),
                };
                if operands.next().is_some() {
                    return Err(ParserError::MissingOperand(line.to_string()).into());
                }
                out.push(parse_instruction("BRH").unwrap().to_string());
                out.push(parse_cond(a)?.to_string());
                out.push(parse_address(addr)?.to_string());
            }
            _ => return Err(ParserError::InvalidInstruction(instruction.to_string()).into()),
        }
        output_lines.push(out.join(" "));
    }

    use std::io::Write;
    let mut output_file = fs::File::create(path.with_extension("mc"))?;
    for l in output_lines {
        writeln!(output_file, "{l}")?
    }
    Ok(())
}

#[allow(clippy::unwrap_used)]
fn parse_cond(a: &str) -> Result<Bits<2>> {
    match a {
        "zero" | "=" => Ok(Bits::from_str("00").unwrap()),
        "notzero" | "!=" => Ok(Bits::from_str("01").unwrap()),
        "carry" | ">=" => Ok(Bits::from_str("10").unwrap()),
        "notcarry" | "<" => Ok(Bits::from_str("11").unwrap()),
        _ => Err(ParserError::InvalidInstruction(a.to_string()).into()),
    }
}

fn parse_address(a: &str) -> Result<Address> {
    Ok(Bits::from_str(a)?)
}

#[allow(clippy::unwrap_used)]
fn parse_instruction(instruction: &str) -> Result<Bits<4>> {
    let instruction_bits: Bits<4> = match instruction {
        "NOP" => Bits::from_str("0000").unwrap(),
        "HLT" => Bits::from_str("0001").unwrap(),
        "ADD" => Bits::from_str("0010").unwrap(),
        "SUB" => Bits::from_str("0011").unwrap(),
        "AND" => Bits::from_str("0100").unwrap(),
        "NOR" => Bits::from_str("0101").unwrap(),
        "XOR" => Bits::from_str("0110").unwrap(),
        "RSH" => Bits::from_str("0111").unwrap(),
        "LDI" => Bits::from_str("1000").unwrap(),
        "ADI" => Bits::from_str("1001").unwrap(),
        "JMP" => Bits::from_str("1010").unwrap(),
        "BRH" => Bits::from_str("1011").unwrap(),
        &_ => return Err(ParserError::InvalidInstruction(instruction.to_string()).into()),
    };
    Ok(instruction_bits)
}

#[cfg(test)]
mod tests;
