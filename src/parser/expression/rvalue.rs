use crate::ir::IR;
use crate::parser::context::Context;
use crate::parser::expression::{
    bin_op::{self, BinOp},
    constant::{self, Constant},
    parenthesis::{self, Parenthesis},
    variable::{self, Variable},
    ExpressionResult,
};
use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;

sum_type! {
    #[derive(Debug, Eq, PartialEq, Clone, Hash)]
    pub enum RValue {
        Constant,
        Variable,
        Parenthesis,
        BinOp,
    }
}

pub fn parse(code: &str) -> IResult<&str, RValue> {
    alt((
        map(bin_op::parse, RValue::BinOp),
        map(constant::parse, RValue::Constant),
        map(variable::parse, RValue::Variable),
        map(parenthesis::parse, RValue::Parenthesis),
    ))(code)
}

impl RValue {
    pub fn ir(&self) -> ExpressionResult {
        match self {
            RValue::Constant(constant) => constant.ir(),
            RValue::Variable(variable) => variable.ir(),
            RValue::Parenthesis(parenthesis) => parenthesis.ir(),
            RValue::BinOp(bin_op) => bin_op.ir(),
        }
    }
}
