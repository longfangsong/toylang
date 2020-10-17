use crate::ir::utils::{local_or_number_literal, LocalOrNumberLiteral};
use crate::shared::parsing;
use nom::bytes::complete::tag;
use nom::character::complete::{multispace0, space0};
use nom::combinator::map;
use nom::multi::separated_list;
use nom::sequence::{delimited, tuple};
use nom::IResult;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Field {
    name: String,
    value: LocalOrNumberLiteral,
}

pub fn parse_field(code: &str) -> IResult<&str, Field> {
    map(
        tuple((
            parsing::ident,
            space0,
            tag(":"),
            space0,
            local_or_number_literal,
        )),
        |(name, _, _, _, value)| Field { name, value },
    )(code)
}

#[derive(Debug, Eq, PartialEq, Clone)]
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
