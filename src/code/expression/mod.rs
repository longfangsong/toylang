use crate::register::SSARegister;
use crate::ssa::SSAStatement;

pub(crate) mod lvalue;
pub(crate) mod rvalue;
mod constant;
mod variable;
pub(crate) mod bin_op;

pub(crate) struct ExpressionResult<'a> {
    pub(crate) ssa_generated: Vec<Box<dyn SSAStatement + 'a>>,
    pub(crate) result: SSARegister,
}
