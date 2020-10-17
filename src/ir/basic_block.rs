use crate::ir::statements::phi::Phi;
use crate::ir::statements::{IRStatement, Terminator};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BasicBlock {
    name: String,
    phis: Vec<Phi>,
    content: Vec<IRStatement>,
    terminator: Option<Terminator>,
}
