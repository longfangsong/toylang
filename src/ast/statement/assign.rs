use crate::{
    ast::expression::{rvalue, rvalue::RValue},
    shared::parsing,
};
use nom::{
    bytes::complete::tag, character::complete::space0, combinator::map, sequence::tuple, IResult,
};

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
