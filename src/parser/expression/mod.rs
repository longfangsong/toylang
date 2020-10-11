use crate::ir::calculate::Operand;
use crate::ir::{Register as LogicalRegister, IR};
use crate::parser::expression::constant::Constant;

pub(crate) mod bin_op;
mod constant;
mod lvalue;
mod parenthesis;
pub(crate) mod rvalue;
pub(crate) mod variable_ref;

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
            ExpressionResult::Complex { result, .. } => Operand::Register((&result).into()),
        }
    }
}
