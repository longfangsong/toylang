use crate::ir::utils::{local, Local};
use nom::{
    bytes::complete::tag,
    character::complete::{space0},
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Ret {
    pub value: Option<Local>,
}

pub fn parse(code: &str) -> IResult<&str, Ret> {
    map(
        tuple((tag("ret"), space0, opt(local::parse))),
        |(_, _, value)| Ret { value },
    )(code)
}
