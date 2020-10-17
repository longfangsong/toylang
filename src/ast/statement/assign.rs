use crate::ast::expression::rvalue;
use crate::ast::expression::rvalue::RValue;
use crate::shared::parsing;
use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Assign {
    lhs: String,
    rhs: RValue,
}

pub fn parse(code: &str) -> IResult<&str, Assign> {
    map(
        tuple((
            parsing::ident,
            space0,
            tag("="),
            space0,
            rvalue::parse,
            space0,
            tag(";"),
        )),
        |(lhs, _, _, _, rhs, _, _)| Assign { lhs, rhs },
    )(code)
}

pub trait AssignVisitor {
    fn visit_assign(&mut self, assign: &Assign);
}
