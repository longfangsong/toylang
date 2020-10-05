use crate::parser::expression::bin_op;
use crate::parser::expression::bin_op::level4::higher_than_level4;
use crate::parser::expression::bin_op::{level4, BinOp};
use crate::parser::expression::rvalue::RValue;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::multi::many1;
use nom::sequence::tuple;
use nom::IResult;

pub(in crate::parser::expression) fn higher_than_level5(code: &str) -> IResult<&str, RValue> {
    alt((map(level4::parse, |binop| binop.into()), higher_than_level4))(code)
}

pub fn parse(code: &str) -> IResult<&str, BinOp> {
    map(
        tuple((
            higher_than_level5,
            many1(map(
                tuple((
                    space0,
                    alt((tag("<<"), tag(">>"))),
                    space0,
                    higher_than_level5,
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
    use crate::parser::expression::constant::Constant;
    use std::convert::TryInto;

    #[test]
    fn it_works() {
        let ast = parse("1+2 >> 3").unwrap().1;
        let (operator, lhs, rhs) = (ast.operator, ast.lhs, ast.rhs);
        assert_eq!(operator, ">>");
        let lhs: BinOp = (*lhs).try_into().unwrap();
        assert_eq!(lhs.operator, "+");
        assert_eq!(rhs, Box::new(Constant(3).into()));
    }
}
