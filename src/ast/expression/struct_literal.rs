use crate::{
    ast::expression::{rvalue, rvalue::RValue},
    shared::parsing,
};
use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, space0},
    combinator::map,
    multi::separated_list,
    sequence::{delimited, tuple},
    IResult,
};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Field {
    name: String,
    value: RValue,
}

pub fn parse_field(code: &str) -> IResult<&str, Field> {
    map(
        tuple((parsing::ident, space0, tag(":"), space0, rvalue::parse)),
        |(name, _, _, _, value)| Field { name, value },
    )(code)
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct StructLiteral {
    name: String,
    fields: Vec<Field>,
}

pub fn parse(code: &str) -> IResult<&str, StructLiteral> {
    map(
        tuple((
            parsing::ident,
            multispace0,
            delimited(
                tuple((multispace0, tag("{"), multispace0)),
                separated_list(tuple((multispace0, tag(","), multispace0)), parse_field),
                tuple((multispace0, tag("}"), multispace0)),
            ),
        )),
        |(name, _, fields)| StructLiteral { name, fields },
    )(code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse() {
        let literal = parse("S { a: 1, b: 2 }").unwrap().1;
        assert_eq!(literal.name, "S");
    }
}
