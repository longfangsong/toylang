use nom::branch::alt;
use nom::IResult;

use crate::code::expression::bin_op::BinOp;
use crate::code::expression::constant::Constant;
use crate::code::expression::ExpressionResult;
use crate::code::expression::variable::Variable;

pub(crate) trait RValue<'a> {
    fn generate_ir(&self) -> ExpressionResult<'a>;
}

pub(crate) fn lift<'a, O: 'a + RValue<'a>>(parser: impl Fn(&'a str) -> IResult<&'a str, O>) -> impl Fn(&'a str) -> IResult<&'a str, Box<dyn RValue<'a> + 'a>> {
    move |code: &'a str| -> IResult<&'a str, Box<dyn RValue<'a> + 'a>> {
        parser(code).map(|(rest, result)| (rest, Box::new(result) as _))
    }
}

pub(crate) fn parse<'a>(code: &'a str) -> IResult<&'a str, Box<dyn RValue + 'a>> {
    alt((lift(BinOp::parse), lift(Constant::parse), lift(Variable::parse)))(code)
}

#[test]
fn test_parse() {
    let result = parse("b+2+d;");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().0, ";");
}

