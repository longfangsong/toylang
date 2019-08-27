use std::ops::RangeTo;

use nom::{IResult, Offset, Slice};
use nom::Err;

use crate::parser::Context;

pub fn lift<I: Copy, O, BeforeLift>(f: BeforeLift) -> impl Fn((I, Context)) -> IResult<(I, Context), O>
    where BeforeLift: Fn(I) -> IResult<I, O> {
    move |(raw, context): (I, Context)| {
        match f(raw) {
            Ok((unparsed, result)) => Ok(((unparsed, context), result)),
            Err(Err::Error((_, kind))) => Err(Err::Error(((raw, context), kind))),
            Err(Err::Incomplete((_))) => panic!("faq"),
            Err(Err::Failure((_))) => panic!("faq")
        }
    }
}