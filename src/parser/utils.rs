use crate::bits::Bits;
use crate::parser::error::ParserError;
use crate::{Address, Immediate, Result};
use std::str::FromStr;

const CHARSET: &str = " abcdefghijklmnopqrstuvwxyz.!?";
const PORTNAMES: [&str; 16] = [
    "pixel_x",
    "pixel_y",
    "draw_pixel",
    "clear_pixel",
    "load_pixel",
    "buffer_screen",
    "clear_screen_buffer",
    "write_char",
    "buffer_chars",
    "clear_chars_buffer",
    "show_number",
    "clear_number",
    "signed_mode",
    "unsigned_mode",
    "rng",
    "controller_input",
];

pub(super) fn parse_register_string(s: &str) -> Result<Bits<4>> {
    if s.len() < 2 || !s.starts_with('r') {
        return Err(ParserError::InvalidInstruction(s.to_string()).into());
    }
    let num_s = &s[1..];
    if num_s.starts_with("-") {
        return Err(ParserError::InvalidInstruction(s.to_string()).into());
    }
    Bits::from_str(num_s)
}

#[allow(clippy::unwrap_used)]
pub(super) fn parse_cond(a: &str) -> Result<Bits<2>> {
    match a.to_lowercase().as_str() {
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
    Bits::from_str(addr)
}

#[allow(clippy::unwrap_used)]
pub(super) fn parse_instruction(instruction: &str) -> Result<Bits<4>> {
    let instruction_bits: Bits<4> = match instruction.to_uppercase().as_str() {
        "NOP" => Bits::from_str("0000").unwrap(),
        "HLT" => Bits::from_str("0001").unwrap(),
        "ADD" => Bits::from_str("0010").unwrap(),
        "SUB" => Bits::from_str("0011").unwrap(),
        "NOR" => Bits::from_str("0100").unwrap(),
        "AND" => Bits::from_str("0101").unwrap(),
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

pub(crate) fn is_definition(line: &str) -> bool {
    line.to_lowercase().starts_with("define")
}

pub(crate) fn is_comment(line: &str) -> bool {
    line.starts_with("//") || line.starts_with('#')
}

// TODO: add program too long error
#[allow(unused_assignments)]
pub(crate) fn find_and_remove_symbols(
    lines: &mut str,
    labels: &mut std::collections::HashMap<String, Address>,
    symbols: &mut std::collections::HashMap<String, Immediate>,
) -> Result<Vec<String>> {
    let mut out = vec![];
    let mut undefined_labels = vec![];
    let mut pc = 0;
    for mut line in lines.lines() {
        line = line.trim();
        if is_label(line) {
            let idx = match line.find(' ') {
                Some(idx) => idx,
                None => line.len(),
            };
            let label = line[..idx].trim_end();
            let rest = line[idx..].trim_start();

            if !rest.is_empty()
                && labels
                    .insert(label.to_string(), Bits::from(pc).resize())
                    .is_some()
            {
                eprintln!("Label '{label}' is redefined");
            } else {
                undefined_labels.push(label.to_string());
            }
            if !rest.is_empty() {
                out.push(rest.to_string());
            }
        } else if is_definition(line) {
            let mut operands = line.split_whitespace();
            operands.next(); // skip "define"
            let ops = extract_n_operands(2, &mut operands, line)?;
            let [name, value] = ops.as_slice() else {
                return Err(ParserError::BadlyDefinedDefinition(line.to_string()).into());
            };
            if symbols
                .insert(name.to_string(), Bits::from_str(value)?)
                .is_some()
            {
                eprintln!("Definition '{name}' is redefined");
            }
            out.push("".to_string());
        } else if is_comment(line) {
            // skip comments
        } else if line.is_empty() {
            // skip empty lines
        } else {
            if !undefined_labels.is_empty() {
                for undefined_label in undefined_labels.iter() {
                    if let Some(_label) =
                        labels.insert(undefined_label.clone(), Bits::from(pc).resize())
                    {
                        // eprint!("redefined addr: {label}");
                    }
                }
            }
            undefined_labels.clear();
            line = line.trim_start();
            out.push(line.to_string());
            pc += 1;
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
        if is_comment(op) {
            break; // stop at comment
        }
        out.push(op);
    }
    Ok(out)
}

//TODO: improve error handling
pub(crate) fn parse_offset(
    offset: &str,
    symbols: &mut std::collections::HashMap<String, Immediate>,
) -> Result<Bits<4>> {
    if let Some(rest) = offset.strip_prefix('-') {
        let num = rest
            .parse::<u8>()
            .map_err(|_| ParserError::InvalidInstruction(offset.to_string()))?;
        let complement = Bits::from(16 - num);
        Ok(complement.resize())
    } else if let Some(value) = symbols.get(offset) {
        Ok(value.resize())
    } else {
        Bits::<4>::from_str(offset)
            .map(|b| b.resize())
            .map_err(|_| ParserError::InvalidInstruction(offset.to_string()).into())
    }
}

pub(crate) fn parse_immediate(
    imm: String,
    symbols: &mut std::collections::HashMap<String, Immediate>,
) -> Result<Bits<8>> {
    // parse chars
    if let Some(stripped) = imm.strip_prefix("\"") {
        if let Some(char) = stripped.strip_suffix("\"") {
            if let Some(idx) = CHARSET.find(&char.to_ascii_lowercase()) {
                return Ok(Bits::from(idx as u8));
            }
        }
    };

    if let Some(stripped) = imm.strip_prefix("\'") {
        if let Some(char) = stripped.strip_suffix("\'") {
            if let Some(idx) = CHARSET.find(&char.to_lowercase()) {
                return Ok(Bits::from(idx as u8));
            }
        }
    };

    // parse port names
    if let Some(idx) = PORTNAMES.iter().position(|&p| p == imm) {
        const PORT_OFFSET: usize = 240;
        let imm = idx + PORT_OFFSET;
        assert!(imm < 256);
        return Ok(Bits::from(imm).resize());
    }

    if let Some(value) = symbols.get(&imm) {
        return Ok(value.resize());
    }
    Bits::from_str(&imm)
}
