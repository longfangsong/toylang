use crate::ast::expression::rvalue;
use crate::ast::expression::rvalue::RValue;
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
