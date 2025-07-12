use crate::alu::Alu;

#[allow(dead_code)]
mod alu;
mod bits;

pub struct VM {
    alu: Alu,
}
