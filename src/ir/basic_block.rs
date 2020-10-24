use crate::{
    ir::{
        statements,
        statements::{phi, phi::Phi, IRStatement, Terminator},
    },
    shared::parsing,
};
use nom::{
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::{map, opt},
    multi::many0,
    sequence::{pair, tuple},
    IResult,
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BasicBlock {
    name: Option<String>,
    phis: Vec<Phi>,
    content: Vec<IRStatement>,
    terminator: Option<Terminator>,
}

fn parse_tag(code: &str) -> IResult<&str, String> {
    map(pair(parsing::ident, tag(":")), |(_, name)| name.to_string())(code)
}

pub fn parse(code: &str) -> IResult<&str, BasicBlock> {
    map(
        tuple((
            opt(parse_tag),
            multispace0,
            many0(map(
                tuple((multispace0, phi::parse, multispace0)),
                |(_, x, _)| x,
            )),
            multispace0,
            many0(map(
                tuple((multispace0, statements::parse_ir_statement, multispace0)),
                |(_, x, _)| x,
            )),
            multispace0,
            opt(map(
                tuple((multispace0, statements::parse_terminator, multispace0)),
                |(_, x, _)| x,
            )),
        )),
        |(name, _, phis, _, content, _, terminator)| BasicBlock {
            name,
            phis,
            content,
            terminator,
        },
    )(code)
}
