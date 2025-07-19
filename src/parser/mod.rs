use std::path::Path;

use crate::Result;
use crate::{bits::Bits, parser::utils::extract_n_operands};
use error::ParserError;
use std::str::FromStr;

use utils::{parse_address, parse_cond, parse_instruction, parse_register_string};

pub(crate) use utils::parse_as_instruction;

pub mod error;
mod utils;

// TODO: improve error handling
#[allow(clippy::unwrap_used)]
pub(crate) fn parse_program(file_path: impl AsRef<Path>) -> Result<()> {
    let mut output_lines = vec![];

    let path = file_path.as_ref();
    let mut content = std::fs::read_to_string(path)
        .map_err(|_| ParserError::FileNotFound(path.display().to_string()))?;

    let mut labels = std::collections::HashMap::new();
    let content = utils::find_and_remove_labels(&mut content, &mut labels)?;
    for line in content.iter() {
        let mut out = vec![];
        let mut splitted = line.split_whitespace();
        let instruction = match splitted.next() {
            Some(instr) => instr,
            None => continue, // Skip empty lines
        };
        let mut operands = splitted;
        match instruction {
            "NOP" | "HLT" | "RET" => {
                let ops = extract_n_operands(0, &mut operands, line)?;
                let [] = ops.as_slice() else {
                    return Err(ParserError::MissingOperand(line.to_string()).into());
                };
                let instruction_bits = parse_instruction(instruction).unwrap();
                out.push(instruction_bits.to_string());
                out.push("0000 0000 0000".to_string());
            }
            "INC" | "DEC" => {
                let ops = extract_n_operands(1, &mut operands, line)?;
                let [r1] = ops.as_slice() else {
                    return Err(ParserError::MissingOperand(line.to_string()).into());
                };
                if instruction == "INC" {
                    out.push(parse_instruction("ADI").unwrap().to_string());
                    out.push(parse_register_string(r1)?.to_string());
                    out.push(Bits::from(1u8).to_string());
                } else {
                    out.push(parse_instruction("ADI").unwrap().to_string());
                    out.push(parse_register_string(r1)?.to_string());
                    out.push(Bits::from(255u8).to_string());
                }
            }
            "JMP" | "CAL" => {
                let ops = extract_n_operands(1, &mut operands, line)?;
                let [addr] = ops.as_slice() else {
                    return Err(ParserError::MissingOperand(line.to_string()).into());
                };
                out.push(parse_instruction(instruction).unwrap().to_string());
                out.push(Bits::<2>::from_str("00").unwrap().to_string());
                out.push(parse_address(addr, &mut labels)?.to_string());
            }
            "CMP" => {
                let ops = extract_n_operands(2, &mut operands, line)?;
                let [r1, r2] = ops.as_slice() else {
                    return Err(ParserError::MissingOperand(line.to_string()).into());
                };
                // CMP rx ry -> SUB rx ry r0
                out.push(parse_instruction("SUB").unwrap().to_string());
                out.push(parse_register_string(r1)?.to_string());
                out.push(parse_register_string(r2)?.to_string());
                out.push("0000".to_string());
            }
            "LDI" | "ADI" => {
                let ops = extract_n_operands(2, &mut operands, line)?;
                let [r1, immediate] = ops.as_slice() else {
                    return Err(ParserError::MissingOperand(line.to_string()).into());
                };
                out.push(parse_instruction(instruction).unwrap().to_string());
                out.push(parse_register_string(r1)?.to_string());
                out.push(Bits::<8>::from_str(immediate)?.to_string());
            }
            "BRH" => {
                let ops = extract_n_operands(2, &mut operands, line)?;
                let [cond, addr] = ops.as_slice() else {
                    return Err(ParserError::MissingOperand(line.to_string()).into());
                };
                out.push(parse_instruction("BRH").unwrap().to_string());
                out.push(parse_cond(cond)?.to_string());
                out.push(parse_address(addr, &mut labels)?.to_string());
            }
            "RSH" => {
                let ops = extract_n_operands(2, &mut operands, line)?;
                let ops_slice = ops.as_slice();
                if let [r1, write] = ops_slice {
                    out.push(parse_instruction("RSH").unwrap().to_string());
                    out.push(parse_register_string(r1)?.to_string());
                    out.push("0000".to_string());
                    out.push(parse_register_string(write)?.to_string());
                } else {
                    return Err(ParserError::MissingOperand(line.to_string()).into());
                }
            }
            "LOD" | "STR" => {
                let ops = extract_n_operands(2, &mut operands, line)?;
                match ops.as_slice() {
                    [r1, addr] => {
                        out.push(parse_instruction(instruction).unwrap().to_string());
                        out.push(parse_register_string(r1)?.to_string());
                        out.push(parse_address(addr, &mut labels)?.to_string());
                    }
                    [r1, r2, offset] => {
                        out.push(parse_instruction(instruction).unwrap().to_string());
                        out.push(parse_register_string(r1)?.to_string());
                        out.push(parse_register_string(r2)?.to_string());
                        out.push(Bits::<8>::from_str(offset)?.resize::<4>().to_string());
                    }
                    _ => return Err(ParserError::MissingOperand(line.to_string()).into()),
                }
            }
            "ADD" | "SUB" | "AND" | "NOR" | "XOR" => {
                let ops = extract_n_operands(3, &mut operands, line)?;
                let [r1, r2, write] = ops.as_slice() else {
                    return Err(ParserError::MissingOperand(line.to_string()).into());
                };
                out.push(parse_instruction(instruction).unwrap().to_string());
                out.push(parse_register_string(r1)?.to_string());
                out.push(parse_register_string(r2)?.to_string());
                out.push(parse_register_string(write)?.to_string());
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
