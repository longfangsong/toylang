use nom::branch::alt;
use nom::IResult;

use crate::parser::Context;
use crate::parser::expression::binary_op::add::parser_add;
use crate::parser::expression::binary_op::div::parser_div;
use crate::parser::expression::binary_op::mul::parser_mul;
use crate::parser::expression::binary_op::rem::parser_rem;
use crate::parser::expression::binary_op::sub::parser_sub;
use crate::parser::expression::number_literal::number_literal;
use crate::parser::expression::rvalue::{as_rvalue_parser, RValue};
use crate::parser::expression::variable_reference::variable_reference;

pub mod add;
pub mod sub;
pub mod mul;
pub mod div;
pub mod rem;

pub(crate) fn priority_0<'a>(input: (&'a str, Context<'a>)) -> IResult<(&'a str, Context<'a>), Box<dyn RValue>> {
    alt((as_rvalue_parser(number_literal), as_rvalue_parser(variable_reference)))(input)
}

pub(crate) fn priority_1<'a>(input: (&'a str, Context<'a>)) -> IResult<(&'a str, Context<'a>), Box<dyn RValue>> {
    alt((as_rvalue_parser(parser_div), as_rvalue_parser(parser_mul), as_rvalue_parser(parser_rem)))(input)
}

pub(crate) fn priority_less_than_1<'a>(input: (&'a str, Context<'a>)) -> IResult<(&'a str, Context<'a>), Box<dyn RValue>> {
    alt((priority_0, priority_1))(input)
}

pub fn binary_op<'a>(input: (&'a str, Context<'a>)) -> IResult<(&'a str, Context<'a>), Box<dyn RValue>> {
    alt((as_rvalue_parser(parser_div), as_rvalue_parser(parser_mul), as_rvalue_parser(parser_rem), as_rvalue_parser(parser_add), as_rvalue_parser(parser_sub)))(input)
}