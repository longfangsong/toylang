use crate::ir::{IR, Register as LogicalRegister};
use crate::parser::expression::constant::Constant;
use crate::ir::calculate::Operand;

pub(crate) mod bin_op;
mod constant;
mod parenthesis;
pub(crate) mod rvalue;
pub(crate) mod variable;
mod lvalue;

pub enum ExpressionResult {
    Constant(i64),
    Complex {
        ir_generated: Vec<IR>,
        result: LogicalRegister,
    },
}

impl From<Constant> for ExpressionResult {
    fn from(constant: Constant) -> Self {
        ExpressionResult::Constant(constant.0)
    }
}

impl Into<Operand> for ExpressionResult {
    fn into(self) -> Operand {
        match self {
            ExpressionResult::Constant(n) => Operand::NumberLiteral(n),
            ExpressionResult::Complex { result, .. } => Operand::Register(result)
        }
    }
}
