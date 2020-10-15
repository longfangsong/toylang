use paste::paste;
macro_rules! bin_op_level {
    ($n: expr, $n_1: expr, $($op: expr)*) => {
        paste! {
            mod [<level $n>] {
                use crate::ast::expression::bin_op::{
                    self, BinOp,
                    [<level $n_1>]::{self, [<higher_than_level $n_1>]}
                };
                use crate::ast::expression::rvalue::RValue;
                use nom::branch::alt;
                use nom::bytes::complete::tag;
                use nom::character::complete::space0;
                use nom::combinator::map;
                use nom::multi::many1;
                use nom::sequence::tuple;
                use nom::IResult;

                pub(in crate::ast::expression) fn [<higher_than_level $n>](
                    code: &str,
                ) -> IResult<&str, RValue> {
                    alt((
                        map([<level $n_1>]::parse, |binop| binop.into()),
                        [<higher_than_level $n_1>],
                    ))(code)
                }

                pub fn parse(code: &str) -> IResult<&str, BinOp> {
                    map(
                        tuple((
                            [<higher_than_level $n>],
                            many1(map(
                                tuple((
                                    space0,
                                    alt((
                                        $(
                                            tag($op),
                                        )*
                                    )),
                                    space0,
                                   [<higher_than_level $n>],
                                )),
                                |(_, op, _, operand)| (op.to_string(), operand),
                            )),
                        )),
                        |(first, rest)| bin_op::to_ast(first, rest),
                    )(code)
                }
            }
        }
    };
}

// levels are same with C's Operator Precedence
mod level3;
bin_op_level!(4, 3, "+" "-");
bin_op_level!(5, 4, "<<" ">>");
bin_op_level!(6, 5, "<=" "<" ">=" ">");
bin_op_level!(7, 6, "==" "!=");
bin_op_level!(8, 7, "&" "&");
bin_op_level!(9, 8, "^" "^");
bin_op_level!(10, 9, "|" "|");

use crate::ast::expression::rvalue::RValue;
use nom::branch::alt;
use nom::IResult;
use std::convert::TryInto;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct BinOp {
    pub operator: String,
    pub lhs: Box<RValue>,
    pub rhs: Box<RValue>,
}

fn to_ast(first: RValue, rest: Vec<(String, RValue)>) -> BinOp {
    let mut current_lhs = first;
    for (op, value) in rest.into_iter() {
        current_lhs = BinOp {
            operator: op,
            lhs: Box::new(current_lhs),
            rhs: Box::new(value),
        }
        .into()
    }
    current_lhs.try_into().unwrap()
}

pub fn parse(code: &str) -> IResult<&str, BinOp> {
    alt((
        level10::parse,
        level9::parse,
        level8::parse,
        level7::parse,
        level6::parse,
        level5::parse,
        level4::parse,
        level3::parse,
    ))(code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn can_parse() {
        let bin_op = parse("s.a + s.b").unwrap().1;
        assert_eq!(bin_op.operator, "+");
    }
}
