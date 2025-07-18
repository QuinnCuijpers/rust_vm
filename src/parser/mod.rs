use std::path::Path;

use crate::bits::Bits;
use crate::Result;
use error::ParserError;
use std::str::FromStr;

use utils::{parse_address, parse_cond, parse_instruction, parse_register_string};

pub(crate) use utils::parse_as_instruction;

pub mod error;
mod utils;

#[allow(clippy::unwrap_used)]
#[allow(unused_variables)] // TODO: remove unused variables
pub(crate) fn parse_program(file_path: impl AsRef<Path>) -> Result<()> {
    let mut output_lines = vec![];

    let path = file_path.as_ref();
    let mut content = std::fs::read_to_string(path)
        .map_err(|_| ParserError::FileNotFound(path.display().to_string()))?;

    let mut labels = std::collections::HashMap::new();
    let content = utils::find_and_remove_labels(&mut content, &mut labels)?;
    for (pc, line) in content.iter().enumerate() {
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
                out.push(parse_address(a, &mut labels)?.to_string());
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
                out.push(parse_address(addr, &mut labels)?.to_string());
            }
            _ => return Err(ParserError::InvalidInstruction(instruction.to_string()).into()),
        }
        output_lines.push(out.join(" "));
    }

    use std::io::Write;
    let mut output_file = std::fs::File::create(path.with_extension("mc"))?;
    for l in output_lines {
        writeln!(output_file, "{l}")?
    }
    Ok(())
}

#[cfg(test)]
mod tests;
