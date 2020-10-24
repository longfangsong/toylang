use crate::shared::{data_type, data_type::Type, parsing};
use nom::{
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::map,
    multi::separated_list,
    sequence::{delimited, pair, tuple},
    IResult,
};

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

pub trait TypeDefinitionVisitor {
    fn visit_type_definition(&mut self, type_definition: &TypeDefinition);
}
