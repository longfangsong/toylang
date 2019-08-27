use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;

use crate::parser::Context;
use crate::parser::expression::binary_op::binary_op;
use crate::parser::expression::number_literal::number_literal;
use crate::parser::expression::result::ExpressionParseResult;
use crate::parser::expression::variable_reference::{variable_reference, VariableReference};

pub trait RValue {
    fn generate_rvalue(&self) -> ExpressionParseResult;
}

pub fn as_rvalue_parser<'a, O: 'static + RValue, Parser>(f: Parser)
                                                         -> impl Fn((&'a str, Context<'a>)) -> IResult<(&'a str, Context<'a>), Box<dyn RValue>>
    where Parser: Fn((&'a str, Context<'a>)) -> IResult<(&'a str, Context<'a>), O> {
    move |input| {
        match f(input) {
            Ok((remain, out)) => Ok((remain, Box::new(out))),
            Err(e) => Err(e)
        }
    }
}

pub fn rvalue<'a>(input: (&'a str, Context<'a>)) -> IResult<(&'a str, Context<'a>), Box<dyn RValue>> {
    alt((as_rvalue_parser(number_literal), as_rvalue_parser(variable_reference), binary_op))(input)
}