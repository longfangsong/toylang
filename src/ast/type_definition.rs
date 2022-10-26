use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, space0},
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, tuple},
    IResult,
};

use crate::utility::{
    data_type::{self, Type},
    parsing,
};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct FieldDefinition {
    name: String,
    data_type: Type,
}

fn parse_field_definition(code: &str) -> IResult<&str, FieldDefinition> {
    map(
        tuple((
            multispace0,
            parsing::ident,
            space0,
            tag(":"),
            space0,
            data_type::parse,
        )),
        |(_, name, _, _, _, data_type)| FieldDefinition { name, data_type },
    )(code)
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct TypeDefinition {
    name: String,
    fields: Vec<FieldDefinition>,
}

pub fn parse(code: &str) -> IResult<&str, TypeDefinition> {
    map(
        tuple((
            multispace0,
            tag("struct"),
            multispace0,
            parsing::ident,
            multispace0,
            delimited(
                tag("{"),
                separated_list0(
                    tuple((multispace0, tag(","), multispace0)),
                    parse_field_definition,
                ),
                tuple((multispace0, tag("}"), multispace0)),
            ),
        )),
        |(_, _, _, name, _, fields)| TypeDefinition { name, fields },
    )(code)
}
