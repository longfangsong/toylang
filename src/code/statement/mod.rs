use nom::branch::alt;
use nom::IResult;

use crate::code::statement::assign::Assign;
use crate::code::statement::compound::Compound;
use crate::ssa::SSAStatement;

mod assign;
mod compound;

pub(crate) type StatementResult<'a> = Vec<Box<dyn SSAStatement + 'a>>;

pub(crate) trait Statement<'a> {
    fn generate_ir(&self) -> StatementResult<'a>;
}

pub(crate) fn lift<'a, O: 'a + Statement<'a>, P>(parser: P) -> impl Fn(&'a str) -> IResult<&'a str, Box<dyn 'a + Statement<'a>>>
    where P: Fn(&'a str) -> IResult<&'a str, O> {
    move |code: &'a str| -> IResult<&'a str, Box<dyn 'a + Statement<'a>>> {
        parser(code).map(|(rest, result)| (rest, Box::new(result) as _))
    }
}

pub(crate) fn parse<'a>(code: &'a str) -> IResult<&'a str, Box<dyn 'a + Statement<'a>>> {
    alt((lift(Assign::parse), lift(Compound::parse)))(code)
}
