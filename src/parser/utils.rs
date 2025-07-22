use crate::bits::Bits;
use crate::parser::error::ParserError;
use crate::{Address, Result};
use std::str::FromStr;

const CHARSET: &str = " abcdefghijklmnopqrstuvwxyz.!?";

pub(super) fn parse_register_string(s: &str) -> Result<Bits<4>> {
    if s.len() < 2 || !s.starts_with('r') {
        return Err(ParserError::InvalidInstruction(s.to_string()).into());
    }
    Ok(Bits::from_str(&s[1..])?)
}

#[allow(clippy::unwrap_used)]
pub(super) fn parse_cond(a: &str) -> Result<Bits<2>> {
    match a {
        "=" | "eq" | "z" | "zero" => Ok(Bits::from_str("00").unwrap()),
        "!=" | "ne" | "nz" | "notzero" => Ok(Bits::from_str("01").unwrap()),
        ">=" | "ge" | "c" | "carry" => Ok(Bits::from_str("10").unwrap()),
        "<" | "lt" | "nc" | "notcarry" => Ok(Bits::from_str("11").unwrap()),
        _ => Err(ParserError::InvalidInstruction(a.to_string()).into()),
    }
}

pub(super) fn parse_address(
    addr: &str,
    labels: &mut std::collections::HashMap<String, Address>,
) -> Result<Address> {
    if addr.starts_with(".") {
        if let Some(addr) = labels.get(addr) {
            return Ok(*addr);
        }
        return Err(ParserError::UndefinedLabel(addr.to_string()).into());
    }
    Ok(Bits::from_str(addr)?)
}

#[allow(clippy::unwrap_used)]
pub(super) fn parse_instruction(instruction: &str) -> Result<Bits<4>> {
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
        "CAL" => Bits::from_str("1100").unwrap(),
        "RET" => Bits::from_str("1101").unwrap(),
        "LOD" => Bits::from_str("1110").unwrap(),
        "STR" => Bits::from_str("1111").unwrap(),
        &_ => return Err(ParserError::InvalidInstruction(instruction.to_string()).into()),
    };
    Ok(instruction_bits)
}

#[allow(clippy::unwrap_used)]
pub(crate) fn parse_as_instruction(line: &str) -> Bits<16> {
    let no_ws: String = line.chars().filter(|c| !c.is_whitespace()).collect();
    Bits::from_str(no_ws.as_str()).unwrap()
}

pub(crate) fn is_label(line: &str) -> bool {
    line.starts_with('.')
}

// TODO: add program too long error
#[allow(unused_assignments)]
pub(crate) fn find_and_remove_labels(
    lines: &mut str,
    labels: &mut std::collections::HashMap<String, Address>,
) -> Result<Vec<String>> {
    let mut out = vec![];
    for (pc, mut line) in lines.lines().enumerate() {
        if is_label(line) {
            if let Some(idx) = line.find(char::is_whitespace) {
                let label = &line[..idx];
                if labels
                    .insert(label.to_string(), Bits::from(pc).resize())
                    .is_some()
                {
                    eprintln!("Label '{label}' is redefined");
                }
                // Mutate line to remove label and following whitespace
                line = line[idx..].trim_start();
                out.push(line.to_string());
            }
        } else {
            line = line.trim_start();
            out.push(line.to_string());
        }
    }
    Ok(out)
}

pub(crate) fn extract_n_operands<'a>(
    n: usize,
    operands: &mut impl Iterator<Item = &'a str>,
    line: &str,
) -> Result<Vec<&'a str>> {
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        match operands.next() {
            Some(op) => out.push(op),
            None => return Err(ParserError::MissingOperand(line.to_string()).into()),
        }
    }
    for op in operands.by_ref() {
        out.push(op);
    }
    Ok(out)
}

//TODO: improve error handling
pub(crate) fn parse_offset(offset: &str) -> Result<Bits<4>> {
    if let Some(rest) = offset.strip_prefix('-') {
        let num = rest
            .parse::<u8>()
            .map_err(|_| ParserError::InvalidInstruction(offset.to_string()))?;
        let complement = Bits::from(16 - num);
        Ok(complement.resize())
    } else {
        Ok(Bits::from_str(offset)?)
    }
}

pub(crate) fn parse_immediate(imm: String) -> Result<Bits<8>> {
    // parse chars
    if let Some(stripped) = imm.strip_prefix("\"") {
        if let Some(char) = stripped.strip_suffix("\"") {
            if let Some(idx) = CHARSET.find(char) {
                return Ok(Bits::from(idx as u8));
            }
        }
    };
    Ok(Bits::from_str(&imm)?)
}
