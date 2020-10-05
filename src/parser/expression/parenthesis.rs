use crate::parser::expression::rvalue::RValue;
use crate::parser::expression::{rvalue, ExpressionResult};
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::delimited;
use nom::IResult;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Parenthesis(pub Box<RValue>);

pub fn parse(code: &str) -> IResult<&str, Parenthesis> {
    map(delimited(tag("("), rvalue::parse, tag(")")), |content| {
        Parenthesis(Box::new(content))
    })(code)
}

impl Parenthesis {
    pub fn ir(&self) -> ExpressionResult {
        self.0.ir()
    }
}
