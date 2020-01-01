use nom::IResult;

use crate::code::expression::ExpressionResult;
use crate::code::expression::variable::Variable;

pub(crate) trait LValue<'a> {
    fn generate_ir(&self) -> ExpressionResult<'a>;
}

fn lift<'a, O: 'a + LValue<'a>, P>(parser: P) -> impl Fn(&'a str) -> IResult<&'a str, Box<dyn LValue<'a> + 'a>>
    where P: Fn(&'a str) -> IResult<&'a str, O> {
    move |code: &str| -> IResult<&str, Box<dyn LValue<'a> + 'a>> {
        parser(code).map(|(rest, result)| (rest, Box::new(result) as _))
    }
}

pub(crate) fn parse<'a>(code: &'a str) -> IResult<&'a str, Box<dyn LValue<'a> + 'a>> {
    lift::<'a>(Variable::parse)(code)
}
