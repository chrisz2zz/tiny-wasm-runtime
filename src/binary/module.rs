use super::{
    instruction::Instruction,
    section::{Function, SectionCode},
    types::FuncType,
};
use nom::{
    bytes::complete::{tag, take},
    number::complete::{le_u32, le_u8},
    sequence::pair,
    IResult,
};
use nom_leb128::leb128_u32;
use num_traits::FromPrimitive as _;

#[derive(Debug, PartialEq, Eq)]
pub struct Module {
    pub magic: String,
    pub version: u32,
    pub type_section: Option<Vec<FuncType>>,
    pub function_section: Option<Vec<u32>>,
    pub code_section: Option<Vec<Function>>,
}

impl Default for Module {
    fn default() -> Self {
        Self {
            magic: "\0asm".to_string(),
            version: 1,
            type_section: None,
            function_section: None,
            code_section: None,
        }
    }
}

impl Module {
    pub fn new(input: &[u8]) -> anyhow::Result<Module> {
        let (_, module) =
            Module::decode(input).map_err(|e| anyhow::anyhow!("failed to parse wasm: {}", e))?;
        Ok(module)
    }

    fn decode(input: &[u8]) -> IResult<&[u8], Module> {
        let (input, _) = tag(b"\0asm")(input)?;
        let (input, version) = le_u32(input)?;

        let mut module = Module {
            magic: "\0asm".into(),
            version,
            ..Default::default()
        };

        let mut remaining = input;

        while !remaining.is_empty() {
            match decode_section_header(remaining) {
                Ok((input, (code, size))) => {
                    let (rest, section_contents) = take(size)(input)?;

                    match code {
                        SectionCode::Type => {
                            let (_, types) = decode_type_section(section_contents)?;
                            module.type_section = Some(types);
                        }
                        SectionCode::Function => {
                            let (_, func_idx_list) = decode_function_section(section_contents)?;
                            module.function_section = Some(func_idx_list);
                        }
                        SectionCode::Code => {
                            let (_, funcs) = decode_code_section(section_contents)?;
                            module.code_section = Some(funcs);
                        }
                        _ => todo!(),
                    };

                    remaining = rest;
                }
                Err(err) => return Err(err),
            }
        }

        Ok((input, module))
    }
}

fn decode_section_header(input: &[u8]) -> IResult<&[u8], (SectionCode, u32)> {
    let (input, (code, size)) = pair(le_u8, leb128_u32)(input)?;
    Ok((
        input,
        (
            SectionCode::from_u8(code).expect("unexpected section code"),
            size,
        ),
    ))
}

fn decode_type_section(_input: &[u8]) -> IResult<&[u8], Vec<FuncType>> {
    let func_types = vec![FuncType::default()];

    // TODO: Decoding arguments and return values

    Ok((&[], func_types))
}

fn decode_function_section(input: &[u8]) -> IResult<&[u8], Vec<u32>> {
    let mut func_idx_list = vec![];
    let (mut input, count) = leb128_u32(input)?;

    for _ in 0..count {
        let (rest, idx) = leb128_u32(input)?;
        func_idx_list.push(idx);
        input = rest;
    }

    Ok((&[], func_idx_list))
}

fn decode_code_section(_input: &[u8]) -> IResult<&[u8], Vec<Function>> {
    // TODO: decoding local variables and instructions

    let functions = vec![Function {
        locals: vec![],
        code: vec![Instruction::End],
    }];

    Ok((&[], functions))
}

#[cfg(test)]
mod tests {
    use crate::binary::{instruction::Instruction, module::Module, section::Function, types::FuncType};
    use anyhow::Result;

    #[test]
    fn decode_simplest_module() -> Result<()> {
        // Generate wasm binary with only preamble present
        let wasm = wat::parse_str("(module)")?;
        // Decode binary and generate Module structure
        let module = Module::new(&wasm)?;
        // Compare whether the generated Module structure is as expected
        assert_eq!(module, Module::default());
        Ok(())
    }

    #[test]
    fn decode_simplest_func() -> Result<()> {
        let wasm = wat::parse_str("(module (func))")?;
        let module = Module::new(&wasm)?;
        assert_eq!(module, Module{
            type_section: Some(vec![FuncType::default()]),
            function_section: Some(vec![0]),
            code_section: Some(vec![Function {
                locals: vec![],
                code: vec![Instruction::End],
            }]),
            ..Default::default()
        });
        Ok(())
    }
}
