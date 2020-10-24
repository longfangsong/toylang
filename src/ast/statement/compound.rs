use crate::ast::statement::{self, Statement};
use nom::{
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::map,
    multi::many0,
    sequence::{delimited, tuple},
    IResult,
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Compound(Vec<Statement>);

pub fn parse(code: &str) -> IResult<&str, Compound> {
    map(
        delimited(
            tag("{"),
            many0(map(
                tuple((multispace0, statement::parse, multispace0)),
                |(_, it, _)| it,
            )),
            tag("}"),
        ),
        Compound,
    )(code)
}
