use crate::bits::Bits;
use std::str::FromStr;

#[allow(clippy::unwrap_used)]
pub(crate) fn parse_as_instruction(line: &str) -> Bits<16> {
    let no_ws: String = line.chars().filter(|c| !c.is_whitespace()).collect();
    Bits::from_str(no_ws.as_str()).unwrap()
}
