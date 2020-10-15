use crate::ast::expression::rvalue;
use crate::ast::expression::rvalue::RValue;
use crate::ast::statement::compound;
use crate::ast::statement::compound::Compound;
use nom::bytes::complete::tag;
use nom::character::complete::{multispace0, space0};
use nom::combinator::{map, opt};
use nom::sequence::tuple;
use nom::IResult;

// todo: else
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct If {
    condition: RValue,
    content: Compound,
    else_content: Option<Compound>,
}

pub fn parse(code: &str) -> IResult<&str, If> {
    map(
        tuple((
            tag("if"),
            space0,
            rvalue::parse,
            space0,
            compound::parse,
            opt(map(
                tuple((multispace0, tag("else"), multispace0, compound::parse)),
                |(_, _, _, else_content)| else_content,
            )),
        )),
        |(_, _, condition, _, content, else_content)| If {
            condition,
            content,
            else_content,
        },
    )(code)
}

pub trait IfVisitor {
    fn visit_if(&mut self, declare: &If);
}
