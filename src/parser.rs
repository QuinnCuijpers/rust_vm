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

pub(crate) fn parse_program(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::path::Path;

    let mut output_lines = vec![];

    let path = Path::new(file_path);
    let content = std::fs::read_to_string(path)
        .map_err(|_| ParserError::FileNotFound(file_path.to_string()))?;
    let lines = content.lines();

    for line in lines {
        let mut out = vec![];
        let splitted = line.split_whitespace().collect::<Vec<&str>>();
        if splitted.is_empty() {
            continue; // Skip empty lines
        }
        if splitted.len() < 2 {
            return Err(Box::new(ParserError::MissingOperand(line.to_string())));
        }
        let instruction = splitted[0];
        let operands = &splitted[1..];
        let instruction_bits = parse_instruction(instruction)?;
        out.push(instruction_bits.to_string());
        if instruction == "RSH" {
            if operands.len() != 2 {
                return Err(Box::new(ParserError::MissingOperand(line.to_string())));
            }
            let a = &operands[0][1..]; // skip first character 'r'
            let write_address = &operands[1][1..]; // skip first character 'r'
            let a_bits: Bits<4> = Bits::from_str(a).unwrap();
            let b_bits: Bits<4> = Bits::from_str("0000").unwrap(); // RSH does not use a second operand
            let write_address_bits: Bits<4> = Bits::from_str(write_address).unwrap();
            out.push(a_bits.to_string());
            out.push(b_bits.to_string());
            out.push(write_address_bits.to_string());
        } else {
            let a = &operands[0][1..]; // skip first character 'r'
            let b = &operands[1][1..]; // skip first character 'r
            let write = &operands[2][1..]; // skip first character 'r'
            let a_bits: Bits<4> = Bits::from_str(a).unwrap();
            let b_bits: Bits<4> = Bits::from_str(b).unwrap();
            let write_bits: Bits<4> = Bits::from_str(write).unwrap();
            out.push(a_bits.to_string());
            out.push(b_bits.to_string());
            out.push(write_bits.to_string());
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
        "ADD" => Bits::from_str("0010")?,
        "SUB" => Bits::from_str("0011")?,
        "AND" => Bits::from_str("0100")?,
        "NOR" => Bits::from_str("0101")?,
        "XOR" => Bits::from_str("0110")?,
        "RSH" => Bits::from_str("0111")?,
        &_ => {
            return Err(Box::new(ParserError::InvalidInstruction(
                instruction.to_string(),
            )))
        }
    };
    Ok(instruction_bits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_program() {
        let result = parse_program("test.as");
        assert!(result.is_ok());
    }
}
