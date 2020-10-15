use crate::ast::expression::rvalue;
use crate::ast::expression::rvalue::RValue;
use crate::ast::statement::compound;
use crate::ast::statement::compound::Compound;
use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

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
