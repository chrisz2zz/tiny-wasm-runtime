use crate::binary::{
    instruction::Instruction,
    module::Module,
    types::{FuncType, ValueType},
};
use anyhow::{bail, Result};

#[derive(Clone, Debug)]
pub struct Func {
    pub locals: Vec<ValueType>,
    pub body: Vec<Instruction>,
}

#[derive(Clone, Debug)]
pub struct InternalFuncInst {
    pub func_type: FuncType,
    pub code: Func,
}

#[derive(Debug, Clone)]
pub enum FuncInst {
    Internal(InternalFuncInst),
}

#[derive(Default, Debug)]
pub struct Store {
    pub funcs: Vec<FuncInst>,
}

impl Store {
    pub fn new(module: Module) -> Result<Self> {
        let func_type_idxs = match module.function_section {
            Some(ref idxs) => idxs.clone(),
            _ => vec![],
        };

        let mut funcs = vec![];

        if let Some(ref code_section) = module.code_section {
            for (func_body, type_idx) in code_section.iter().zip(func_type_idxs.into_iter()) {
                let Some(ref func_types) = module.type_section else {
                    bail!("not found type_section")
                };

                let Some(func_type) = func_types.get(type_idx as usize) else {
                    bail!("not found func type in type_section")
                };

                let mut locals = Vec::with_capacity(func_body.locals.len());
                for local in func_body.locals.iter() {
                    for _ in 0..local.type_count {
                        locals.push(local.value_type.clone());
                    }
                }

                let func = FuncInst::Internal(InternalFuncInst {
                    func_type: func_type.clone(),
                    code: Func {
                        locals,
                        body: func_body.code.clone(),
                    },
                });
                funcs.push(func);
            }
        }

        Ok(Self { funcs })
    }
}
