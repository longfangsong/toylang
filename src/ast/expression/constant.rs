use crate::ast::context::Context;
use crate::ast::expression::ExpressionResult;
use crate::ir::{Alloca, IR};
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::IResult;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Constant(pub i64);

impl From<i64> for Constant {
    fn from(i: i64) -> Self {
        Constant(i)
    }
}

// todo: 支持负数
pub fn parse(code: &str) -> IResult<&str, Constant> {
    map(digit1, |digits| {
        Constant(i64::from_str_radix(digits, 10).unwrap())
    })(code)
}

impl Constant {
    pub fn ir(&self) -> ExpressionResult {
        ExpressionResult::Constant(self.0)
    }
}
