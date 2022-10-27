use crate::{
    ir::{
        basic_block,
        basic_block::BasicBlock,
        utils::{local, Local},
    },
    utility::{data_type, data_type::Type, parsing},
};
use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, space0},
    combinator::map,
    multi::{many0, separated_list0},
    sequence::{delimited, tuple},
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
                separated_list0(tuple((multispace0, tag(","), multispace0)), parse_parameter),
                tag(")"),
            ),
            multispace0,
            tag("->"),
            multispace0,
            data_type::parse,
            multispace0,
            delimited(tag("{"), many0(basic_block::parse), tag("}")),
        )),
        |(_, _, name, parameters, _, _, _, return_type, _, basic_blocks)| FunctionDefinition {
            name,
            parameters,
            return_type,
            content: basic_blocks,
        },
    )(code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse() {
        let code = "fn reduce(%s: S) -> i32 {
    %0 = loadfield i32 %s, 0
    %1 = loadfield i32 %s, 1
    %2 = add i32 %0, %1
    ret %2
}";
        let function_definition = parse(code).unwrap().1;
        println!("{:?}", function_definition);
        let code = "fn main() -> () {
    %1 = alloca i32
    store i32 1, address %1
    %2 = alloca i32
    store i32 2, address %2
    %3 = alloca i32
    %4 = load i32 %1
    %5 = load i32 %2
    %6 = add i32 %3, %4
WHILE_0_JUDGE:
    %7 = load i32 @g
    blt 0, %7, WHILE_0_TRUE, WHILE_0_FALSE
WHILE_0_TRUE:
    %8 = load i32 %3
    %9 = load i32 %1
    %10 = sub i32 %8, %9
    %11 = load i32 @g
    %12 = sub i32 %11, 1
    store i32 %12, address @g
    j WHILE_0_JUDGE
WHILE_0_FALSE:
    %13 = load i32 @g
    blt 0, %13, IF_0_TRUE, IF_0_FALSE
IF_0_TRUE:
    %14 = load i32 %1
    store i32 %14, address %2
    j IF_0_END
IF_0_FALSE:
    %14 = load i32 %1
    store i32 %14, address %2
    j IF_0_END
IF_0_END:
    ret
}";
        let function_definition = parse(code).unwrap().1;
        println!("{:?}", function_definition);
    }
}
