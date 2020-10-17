use crate::ir::basic_block::BasicBlock;
use crate::shared::data_type::Type;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, multispace0, space0};
use nom::combinator::map;
use nom::multi::separated_list;
use nom::sequence::{delimited, pair, tuple};
use nom::IResult;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Parameter {
    name: String,
    data_type: Type,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FunctionDefinition {
    name: String,
    parameters: Vec<Parameter>,
    return_type: Type,
    content: Vec<BasicBlock>,
}

fn parse(code: &str) -> IResult<&str, FunctionDefinition> {
    map(
        tuple((
            tag("fn"),
            space0,
            parse::ident,
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
