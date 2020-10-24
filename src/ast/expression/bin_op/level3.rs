use crate::ast::expression::{
    bin_op, bin_op::BinOp, field, integer_literal, parenthesis, rvalue::RValue, variable_ref,
};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::space0, combinator::map, multi::many1,
    sequence::tuple, IResult,
};

pub(in crate::ast::expression) fn higher_than_level3(code: &str) -> IResult<&str, RValue> {
    alt((
        map(field::parse, RValue::Field),
        map(integer_literal::parse, RValue::IntegerLiteral),
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
    use crate::ast::expression::integer_literal::IntegerLiteral;
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
                lhs: Box::new(IntegerLiteral(1).into()),
                rhs: Box::new(IntegerLiteral(2).into()),
            }
        );
        assert_eq!(rhs, Box::new(IntegerLiteral(3).into()));
    }
}
