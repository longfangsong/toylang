use crate::ir::{Register, RegisterRef, IR};
use crate::riscv::ir_translator;
use crate::riscv::register::PhysicalRegister;
use std::collections::HashMap;

mod alloca;
mod branch;
mod calculate;
mod jump;
mod load;
mod store;

fn translate_ir(ir: &IR, registers: &HashMap<RegisterRef, PhysicalRegister>) -> String {
    match ir {
        IR::Store(store) => store.generate_asm(registers),
        IR::Load(load) => load.generate_asm(registers),
        IR::Calculate(calculate) => calculate.generate_asm(registers),
        IR::Branch(branch) => branch.generate_asm(registers),
        IR::Jump(jump) => jump.generate_asm(registers),

        IR::Alloca(_alloca) => unreachable!(),
        IR::Global(_global) => unreachable!(),
    }
}

pub fn translate_irs(irs: &[IR], registers: &HashMap<RegisterRef, PhysicalRegister>) -> String {
    irs.iter()
        .map(|ir| ir_translator::translate_ir(ir, registers))
        .collect::<Vec<_>>()
        .join("\n")
}
