use crate::ast::expression::{rvalue, rvalue::RValue};
use nom::{
    bytes::complete::tag,
    character::complete::{space0, space1},
    combinator::map,
    sequence::tuple,
    IResult,
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Return(RValue);

pub fn parse(code: &str) -> IResult<&str, Return> {
    map(
        tuple((tag("return"), space1, rvalue::parse, space0, tag(";"))),
        |(_, _, value, _, _)| Return(value),
    )(code)
}

pub trait ReturnVisitor {
    fn visit_return(&mut self, declare: &Return);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::expression::bin_op::BinOp;
    use std::convert::TryInto;

    #[test]
    fn can_parse() {
        let return_statement = parse("return s.a + s.b;").unwrap().1;
        let exp: BinOp = return_statement.0.try_into().unwrap();
        assert_eq!(exp.operator, "+");
    }
}
