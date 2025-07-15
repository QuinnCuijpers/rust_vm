#![allow(dead_code)]
use crate::{alu::Alu, register::RegisterFile};
mod alu;
mod bits;
mod control_rom;
mod parser;
mod register;

#[derive(Debug)]
pub struct VM {
    alu: Alu,
    registerbank: RegisterFile,
}
