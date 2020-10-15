use crate::shared::data_type::Type;
use crate::shared::{data_type, parse};
use nom::bytes::complete::tag;
use nom::character::complete::{multispace0, space0};
use nom::combinator::map;
use nom::multi::separated_list;
use nom::sequence::{delimited, tuple};
use nom::IResult;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FieldDefinition {
    name: String,
    data_type: Type,
}

fn parse_field_definition(code: &str) -> IResult<&str, FieldDefinition> {
    map(
        tuple((
            multispace0,
            parse::ident,
            space0,
            tag(":"),
            space0,
            data_type::parse_type,
        )),
        |(_, name, _, _, _, data_type)| FieldDefinition { name, data_type },
    )(code)
}

#[derive(Debug, Eq, PartialEq, Clone)]
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
            parse::ident,
            multispace0,
            delimited(
                tag("{"),
                separated_list(
                    tuple((multispace0, tag(","), multispace0)),
                    parse_field_definition,
                ),
                tuple((multispace0, tag("}"), multispace0)),
            ),
        )),
        |(_, _, _, name, _, fields)| TypeDefinition { name, fields },
    )(code)
}

pub trait TypeDefinitionVisitor {
    fn visit_type_definition(&mut self, type_definition: &TypeDefinition);
}
