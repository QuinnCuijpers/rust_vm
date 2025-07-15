#![allow(dead_code)]
use crate::alu::Alu;
mod alu;
mod bits;
mod parser;
mod register;

#[derive(Debug)]
pub struct VM {
    alu: Alu,
}
