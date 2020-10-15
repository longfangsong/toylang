use crate::ast::statement::{self, Statement};
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::{delimited, tuple};
use nom::IResult;

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
