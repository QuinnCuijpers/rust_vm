use crate::alu::Alu;

#[allow(dead_code)]
mod alu;
mod bits;
mod register;

#[derive(Debug)]
pub struct VM {
    alu: Alu,
}
