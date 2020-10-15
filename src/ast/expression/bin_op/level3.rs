use crate::ast::expression::bin_op::BinOp;
use crate::ast::expression::rvalue::RValue;
use crate::ast::expression::{bin_op, constant, field, parenthesis, variable_ref};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::multi::many1;
use nom::sequence::tuple;
use nom::IResult;

pub(in crate::ast::expression) fn higher_than_level3(code: &str) -> IResult<&str, RValue> {
    alt((
        map(field::parse, RValue::Field),
        map(constant::parse, RValue::Constant),
        map(variable_ref::parse, RValue::VariableRef),
        map(parenthesis::parse, RValue::Parenthesis),
    ))(code)
}

pub fn parse(code: &str) -> IResult<&str, BinOp> {
    map(
        tuple((
            higher_than_level3,
            many1(map(
                tuple((
                    space0,
                    alt((tag("*"), tag("/"))),
                    space0,
                    higher_than_level3,
                )),
                |(_, op, _, operand)| (op.to_string(), operand),
            )),
        )),
        |(first, rest)| bin_op::to_ast(first, rest),
    )(code)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::expression::constant::Constant;
    use std::convert::TryInto;

    #[test]
    fn it_works() {
        let ast = parse("1*2/3").unwrap().1;
        let (operator, lhs, rhs) = (ast.operator, ast.lhs, ast.rhs);
        assert_eq!(operator, "/");
        let lhs: BinOp = (*lhs).try_into().unwrap();
        assert_eq!(
            lhs,
            BinOp {
                operator: "*".to_string(),
                lhs: Box::new(Constant(1).into()),
                rhs: Box::new(Constant(2).into()),
            }
        );
        assert_eq!(rhs, Box::new(Constant(3).into()));
    }
}
