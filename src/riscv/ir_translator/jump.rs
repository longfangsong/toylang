use crate::ir;
use crate::ir::{Register as LogicalRegister, RegisterRef};
use crate::riscv::register::PhysicalRegister;
use std::collections::HashMap;

impl ir::Jump {
    pub(crate) fn generate_asm(
        &self,
        _register_map: &HashMap<RegisterRef, PhysicalRegister>,
    ) -> String {
        format!("j {}", self.label)
    }
}
