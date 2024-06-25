use num_derive::FromPrimitive;

use super::{instruction::Instruction, types::FunctionLocal};

#[derive(Debug, PartialEq, Eq, FromPrimitive)]
pub enum SectionCode {
    Custom = 0x00,
    Type = 0x01,
    Import = 0x02,
    Function = 0x03,
    Memory = 0x05,
    Export = 0x07,
    Code = 0x0A,
    Data = 0x0B,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub locals: Vec<FunctionLocal>,
    pub code: Vec<Instruction>,
}
