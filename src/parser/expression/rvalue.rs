use nom::branch::alt;
use nom::IResult;

use crate::parser::expression::{bin_op, constant, variable};

pub trait RValue: std::fmt::Debug {
    fn generate_rvalue_ir(&self) -> (String, u64);
}

// todo: create a box_result to abstract rvalue::lift and lvalue::lift
pub(crate) fn lift<'a, O: 'a + RValue, P>(parser: P) -> impl Fn(&'a str) -> IResult<&'a str, Box<dyn 'a + RValue>>
    where P: Fn(&'a str) -> IResult<&'a str, O> {
    move |code: &'a str| -> IResult<&'a str, Box<dyn 'a + RValue>> {
        parser(code).map(|(rest, result)| (rest, Box::new(result) as _))
    }
}

pub fn parse<'a>(code: &'a str) -> IResult<&'a str, Box<dyn 'a + RValue>> {
    alt((lift(bin_op::parse), lift(constant::parse), lift(variable::parse)))(code)
}