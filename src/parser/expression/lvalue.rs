use nom::IResult;

use crate::parser::Context;
use crate::parser::expression::result::ExpressionParseResult;
use crate::parser::expression::variable_reference::variable_reference;

pub trait LValue {
    fn generate_lvalue(self) -> ExpressionParseResult;
}

pub fn lvalue<'a>(input: (&'a str, Context<'a>)) -> IResult<(&'a str, Context<'a>), impl LValue> {
    variable_reference(input)
}