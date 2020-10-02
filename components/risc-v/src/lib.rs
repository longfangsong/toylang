#[macro_use]
extern crate lazy_static;

use crate::ir_wrapper::RiscVIR;
use crate::register::AllocatedRegister;
use ir::IR;
use std::collections::HashMap;

mod ir_wrapper;
mod register;

fn compile_single_global(ir: &IR) -> String {
    if let IR::Global(global) = ir {
        format!(".{}: .word {}", global.name, global.initial_value)
    } else {
        unreachable!();
    }
}

fn compile_global(irs: Vec<IR>) -> String {
    ".data\n".to_string()
        + &irs
            .iter()
            .map(compile_single_global)
            .collect::<Vec<_>>()
            .join("\n")
}

fn compile_alloca(irs: Vec<IR>, registers: &HashMap<String, AllocatedRegister>) -> String {
    if let Some(save_space) = irs
        .iter()
        .map(|it| it.created())
        .filter_map(|it| it)
        .filter_map(|it| registers.get(it))
        .map(|register| {
            if let AllocatedRegister::Memory(address) = register {
                *address
            } else {
                0
            }
        })
        .max()
    {
        format!(
            "addi sp, sp, -{}\naddi s0, sp, {}",
            save_space + 4,
            save_space + 4
        )
    } else {
        String::new()
    }
}

fn make_sure_real_register(register: AllocatedRegister, current_used: usize) -> (String, usize) {
    if let AllocatedRegister::Memory(offset) = register {
        (
            format!("lw t{}, {}(sp)", current_used, offset),
            current_used,
        )
    } else {
        ("".to_string(), current_used + 1)
    }
}

fn compile_single_code(ir: IR, registers: &HashMap<String, AllocatedRegister>) -> String {
    match ir {
        IR::Store(store) => store.generate_asm(registers),
        IR::Load(load) => load.generate_asm(registers),
        IR::Calculate(calculate) => calculate.generate_asm(registers),
        IR::Branch(branch) => branch.generate_asm(registers),
        IR::Jump(jump) => jump.generate_asm(registers),
        IR::Label(label) => format!("{}", label),

        IR::Alloca(alloca) => unreachable!(),
        IR::Global(global) => unreachable!(),
    }
}

fn compile_code(irs: Vec<IR>, registers: &HashMap<String, AllocatedRegister>) -> String {
    irs.into_iter()
        .map(|ir| compile_single_code(ir, registers))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn compile(irs: Vec<IR>) -> String {
    // todo: compile in functions
    let registers = register::allocate_registers(&irs);
    let (global, code): (Vec<IR>, Vec<IR>) =
        irs.into_iter()
            .partition(|x| if let IR::Global(_) = x { true } else { false });
    let (alloca, code): (Vec<IR>, Vec<IR>) =
        code.into_iter()
            .partition(|x| if let IR::Alloca(_) = x { true } else { false });
    let alloca_code = compile_alloca(alloca, &registers);
    let code = compile_code(code, &registers);
    let global = compile_global(global);
    alloca_code + "\n" + &code + "\n" + &global
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::character::complete::{line_ending, multispace0};
    use nom::combinator::{map, opt};
    use nom::multi::many0;
    use nom::sequence::tuple;
    use nom::IResult;

    #[test]
    fn it_works() {
        let code = include_str!("../test.ir");
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
