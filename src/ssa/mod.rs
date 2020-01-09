use std::collections::HashMap;

use crate::register::{PhysicalRegister, SSARegister};

pub(crate) mod load_address;
pub(crate) mod load_instant;
pub(crate) mod load_variable;
pub(crate) mod bin_op;
pub(crate) mod store;
pub(crate) mod branch;
pub(crate) mod label;

pub(crate) trait SSAStatement: std::fmt::Display {
    fn require_registers(&self) -> Vec<SSARegister>;
    fn generate_asm(&self, register_map: &HashMap<SSARegister, PhysicalRegister>) -> String;
}