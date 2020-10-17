use crate::ast::statement;
use crate::ast::statement::{Statement, StatementVisitor};
use crate::shared::data_type;
use crate::shared::data_type::Type;
use crate::shared::parsing;
use nom::bytes::complete::tag;
use nom::character::complete::{multispace0, space0};
use nom::combinator::map;
use nom::multi::separated_list;
use nom::sequence::{delimited, pair, tuple};
use nom::IResult;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Parameter {
    name: String,
    data_type: Type,
}

pub fn parse_parameter(code: &str) -> IResult<&str, Parameter> {
    map(
        tuple((parsing::ident, space0, tag(":"), space0, data_type::parse)),
        |(name, _, _, _, data_type)| Parameter { name, data_type },
    )(code)
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FunctionDefinition {
    name: String,
    parameters: Vec<Parameter>,
    return_type: Type,
    content: Vec<Statement>,
}

pub fn parse(code: &str) -> IResult<&str, FunctionDefinition> {
    map(
        tuple((
            tag("fn"),
            space0,
            parsing::ident,
            delimited(
                pair(tag("("), space0),
                separated_list(tuple((space0, tag(","), space0)), parse_parameter),
                pair(space0, tag(")")),
            ),
            space0,
            tag("->"),
            space0,
            data_type::parse,
            delimited(
                tuple((multispace0, tag("{"), multispace0)),
                separated_list(multispace0, statement::parse),
                tuple((multispace0, tag("}"), multispace0)),
            ),
        )),
        |(_, _, name, parameters, _, _, _, return_type, content)| FunctionDefinition {
            name,
            parameters,
            return_type,
            content,
        },
    )(code)
}

pub trait FunctionDefinitionVisitor: StatementVisitor {
    fn visit_function_definition(&mut self, function_definition: &FunctionDefinition);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse() {
        let function_definition = parse(
            "fn add(a: i32, b: i32) -> i32 {
    return a + b;
}",
        )
        .unwrap()
        .1;
        assert_eq!(function_definition.name, "add");
        assert_eq!(function_definition.parameters.len(), 2);
    }
}
