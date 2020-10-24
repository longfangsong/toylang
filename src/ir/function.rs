use crate::{
    ir::{
        basic_block,
        basic_block::BasicBlock,
        utils::{local, Local},
    },
    shared::{data_type, data_type::Type, parsing},
};
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, multispace0, space0},
    combinator::map,
    multi::separated_list,
    sequence::{delimited, pair, tuple},
    IResult,
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Parameter {
    name: Local,
    data_type: Type,
}

fn parse_parameter(code: &str) -> IResult<&str, Parameter> {
    map(
        tuple((local::parse, space0, tag(":"), space0, data_type::parse)),
        |(name, _, _, _, data_type)| Parameter { name, data_type },
    )(code)
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FunctionDefinition {
    name: String,
    parameters: Vec<Parameter>,
    return_type: Type,
    content: Vec<BasicBlock>,
}

pub fn parse(code: &str) -> IResult<&str, FunctionDefinition> {
    map(
        tuple((
            tag("fn"),
            space0,
            parsing::ident,
            delimited(
                tag("("),
                separated_list(tuple((multispace0, tag(","), multispace0)), parse_parameter),
                tag(")"),
            ),
            multispace0,
            tag("->"),
            multispace0,
            data_type::parse,
            multispace0,
            delimited(
                tag("{"),
                separated_list(pair(multispace0, line_ending), basic_block::parse),
                tag("}"),
            ),
        )),
        |(_, _, name, parameters, _, _, _, return_type, _, basic_blocks)| FunctionDefinition {
            name,
            parameters,
            return_type,
            content: basic_blocks,
        },
    )(code)
}

pub trait FunctionDefinitionVisitor {
    fn visit_function_definition(&mut self, function_definition: &FunctionDefinition);
}
