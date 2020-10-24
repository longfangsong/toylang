use crate::ast::{
    expression::{rvalue, rvalue::RValue},
    statement::{compound, compound::Compound},
};
use nom::{
    bytes::complete::tag, character::complete::space0, combinator::map, sequence::tuple, IResult,
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct While {
    condition: RValue,
    content: Compound,
}

pub fn parse(code: &str) -> IResult<&str, While> {
    map(
        tuple((tag("while"), space0, rvalue::parse, space0, compound::parse)),
        |(_, _, condition, _, content)| While { condition, content },
    )(code)
}

pub trait WhileVisitor {
    fn visit_while(&mut self, declare: &While);
}
