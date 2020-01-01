use nom::branch::alt;
use nom::character::complete::{line_ending, space1};
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::tuple;

use crate::code::statement::{Statement, StatementResult};

pub(crate) mod expression;
mod statement;

pub(crate) fn generate_ir(code: &str) -> StatementResult {
    let parse_with_space = map(tuple((
        many0(alt((line_ending, space1))),
        statement::parse,
        many0(alt((line_ending, space1))))), |(_, r, _)| r);
    let statements = many0(parse_with_space)(code).unwrap().1;
    statements.iter()
        .map(|it: &Box<dyn Statement>| it.generate_ir())
        .flatten()
        .collect()
}