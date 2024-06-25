use num_derive::FromPrimitive;

#[derive(Debug, FromPrimitive, PartialEq)]
pub enum Opcode {
    End = 0x0b,
    LocalGet = 0x20,
    I32Add = 0x6A,
    Call = 0x10,
}
