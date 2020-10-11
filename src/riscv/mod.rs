use crate::ir::register::Register as LogicalRegister;
use crate::ir::register::RegisterRef as LogicalRegisterRef;
use crate::ir::IR;
use crate::riscv::register::PhysicalRegister;
use std::collections::HashMap;

// use crate::ir::IR;
// use register::PhysicalRegister;
// use std::collections::HashMap;
//
mod ir_translator;
mod register;

fn compile_global(ir: &IR) -> String {
    if let IR::Global(global) = ir {
        format!(".{}: .word {}", global.name, global.initial_value)
    } else {
        unreachable!();
    }
}

fn compile_globals(irs: &[IR]) -> String {
    ".data\n".to_string()
        + &irs
            .iter()
            .map(compile_global)
            .collect::<Vec<_>>()
            .join("\n")
}

fn compile_alloca(
    irs: &[IR],
    registers: &mut HashMap<LogicalRegisterRef, PhysicalRegister>,
) -> String {
    let mut alloc_space = 0;
    for ir in irs {
        if let IR::Alloca(alloca) = ir {
            alloc_space += alloca.alloc_space();
            registers.insert((&(alloca.to)).into(), PhysicalRegister::Memory(alloc_space));
        }
    }
    alloc_space += irs
        .iter()
        .map(|it| it.create_register())
        .filter_map(|it| it)
        .filter_map(|it| registers.get(&it.into()))
        .chain(registers.values())
        .map(|register| {
            if let PhysicalRegister::Memory(address) = register {
                *address
            } else {
                0
            }
        })
        .max()
        .unwrap_or(0);
    if alloc_space != 0 {
        format!(
            "addi sp, sp, -{}\naddi s0, sp, {}",
            alloc_space, alloc_space
        )
    } else {
        "".to_string()
    }
}

pub fn compile(irs: Vec<IR>) -> String {
    // todo: compile in functions
    let mut registers = register::allocate_registers(&irs);
    let (global, code): (Vec<IR>, Vec<IR>) =
        irs.into_iter().partition(|x| matches!(x, IR::Global(_)));
    let (alloca, code): (Vec<IR>, Vec<IR>) =
        code.into_iter().partition(|x| matches!(x, IR::Alloca(_)));
    let alloca_code = compile_alloca(&alloca, &mut registers);
    let code = ir_translator::translate_irs(&code, &registers);
    let global = compile_globals(&global);
    alloca_code + "\n" + &code + "\n" + &global
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir;
    use nom::character::complete::{line_ending, multispace0};
    use nom::combinator::{map, opt};
    use nom::multi::many0;
    use nom::sequence::tuple;
    use nom::IResult;

    #[test]
    fn it_works() {
        let code = include_str!("./test.ir");
        let parser = |code: &'static str| -> IResult<&'static str, Vec<IR>> {
            many0(map(
                tuple((multispace0, ir::ir, opt(line_ending))),
                |(_, result, _)| result,
            ))(code)
        };
        let (_, ir) = parser(code).unwrap();
        println!("{}", compile(ir));
    }
}
