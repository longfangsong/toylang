use crate::ast::expression::{rvalue, rvalue::RValue};
use nom::{bytes::complete::tag, combinator::map, sequence::delimited, IResult};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Parenthesis(pub Box<RValue>);

pub fn parse(code: &str) -> IResult<&str, Parenthesis> {
    map(delimited(tag("("), rvalue::parse, tag(")")), |content| {
        Parenthesis(Box::new(content))
    })(code)
}
