use crate::ir::register::Register as LogicalRegister;
use crate::ir::IR;
use register::PhysicalRegister;
use std::collections::HashMap;

mod ir_translator;
mod register;

fn compile_global(ir: &&IR) -> String {
    if let IR::Global(global) = ir {
        format!(".{}: .word {}", global.name, global.initial_value)
    } else {
        unreachable!();
    }
}

fn compile_globals(irs: Vec<&IR>) -> String {
    ".data\n".to_string()
        + &irs
            .iter()
            .map(compile_global)
            .collect::<Vec<_>>()
            .join("\n")
}

fn compile_alloca(
    irs: Vec<&IR>,
    registers: &HashMap<&LogicalRegister, PhysicalRegister>,
) -> String {
    if let Some(save_space) = irs
        .iter()
        .map(|it| it.create_register())
        .filter_map(|it| it)
        .filter_map(|it| registers.get(it))
        .chain(registers.values())
        .map(|register| {
            if let PhysicalRegister::Memory(address) = register {
                *address
            } else {
                0
            }
        })
        .max()
    {
        format!("addi sp, sp, -{}\naddi s0, sp, {}", save_space, save_space)
    } else {
        String::new()
    }
}

pub fn compile(irs: Vec<IR>) -> String {
    // todo: compile in functions
    let registers = register::allocate_registers(&irs);
    let (global, code): (Vec<&IR>, Vec<&IR>) =
        irs.iter()
            .partition(|x| if let IR::Global(_) = x { true } else { false });
    let (alloca, code): (Vec<&IR>, Vec<&IR>) =
        code.into_iter()
            .partition(|x| if let IR::Alloca(_) = x { true } else { false });
    let alloca_code = compile_alloca(alloca, &registers);
    let code = ir_translator::translate_irs(code, &registers);
    let global = compile_globals(global);
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
