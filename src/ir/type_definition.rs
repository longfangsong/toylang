use crate::utility::{data_type, data_type::Type, parsing};
use nom::{
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, tuple},
    IResult,
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypeDefinition {
    pub name: String,
    pub fields: Vec<Type>,
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
                tuple((multispace0, tag("{"), multispace0)),
                separated_list0(
                    tuple((multispace0, tag(","), multispace0)),
                    data_type::parse,
                ),
                tuple((multispace0, tag("}"), multispace0)),
            ),
        )),
        |(_, name, _, _, _, _, _, fields)| TypeDefinition { name, fields },
    )(code)
}

pub trait TypeDefinitionVisitor {
    fn visit_type_definition(&mut self, type_definition: &TypeDefinition);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse() {
        let code = "%S = type {\n
    i32,\n
    i32\n
}";
        let result = parse(code).unwrap().1;
        println!("{:?}", result);
    }
}
