use crate::shared::data_type::Type;
use crate::shared::{data_type, parsing};
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::multi::separated_list;
use nom::sequence::{delimited, pair, tuple};
use nom::IResult;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypeDefinition {
    name: String,
    fields: Vec<Type>,
}

pub fn parse(code: &str) -> IResult<&str, TypeDefinition> {
    map(
        tuple((
            tag("%"),
            parsing::ident,
            multispace0,
            tag("="),
            multispace0,
            tag("type"),
            multispace0,
            delimited(
                tag("{"),
                separated_list(
                    tuple((multispace0, tag(","), multispace0)),
                    data_type::parse,
                ),
                pair(multispace0, tag("}")),
            ),
        )),
        |(_, name, _, _, _, _, _, fields)| TypeDefinition { name, fields },
    )(code)
}
