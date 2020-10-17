use crate::ast::expression::rvalue::RValue;
use crate::ast::expression::{bin_op, integer_literal, parenthesis, rvalue, variable_ref};
use crate::shared::parsing;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::multi::separated_list;
use nom::sequence::{delimited, tuple};
use nom::IResult;

fn higher_than_function(code: &str) -> IResult<&str, RValue> {
    alt((
        map(bin_op::parse, RValue::BinOp),
        map(integer_literal::parse, RValue::IntegerLiteral),
        map(variable_ref::parse, RValue::VariableRef),
        map(parenthesis::parse, RValue::Parenthesis),
    ))(code)
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: Vec<RValue>,
}

fn parse_function_like_call(code: &str) -> IResult<&str, FunctionCall> {
    map(
        tuple((
            parsing::ident,
            delimited(
                tag("("),
                separated_list(tuple((space0, tag(","), space0)), higher_than_function),
                tag(")"),
            ),
        )),
        |(name, arguments)| FunctionCall { name, arguments },
    )(code)
}

fn parse_method_like_call(code: &str) -> IResult<&str, FunctionCall> {
    map(
        tuple((
            higher_than_function,
            tag("."),
            parsing::ident,
            delimited(
                tag("("),
                separated_list(tuple((space0, tag(","), space0)), rvalue::parse),
                tag(")"),
            ),
        )),
        |(argument0, _, name, mut arguments)| {
            arguments.insert(0, argument0);
            FunctionCall { name, arguments }
        },
    )(code)
}

pub fn parse(code: &str) -> IResult<&str, FunctionCall> {
    alt((parse_function_like_call, parse_method_like_call))(code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse() {
        // let function_call = parse("f()").unwrap().1;
        // assert_eq!(function_call.name, "f");
        // let function_call = parse("f(a,b)").unwrap().1;
        // assert_eq!(function_call.name, "f");
        // let function_call = parse("f(a+b,c)").unwrap().1;
        // assert_eq!(function_call.name, "f");
        // let function_call = parse("a.b()").unwrap().1;
        // assert_eq!(function_call.name, "b");
        // let function_call = parse("a.b(c)").unwrap().1;
        // assert_eq!(function_call.name, "b");
        // let function_call = parse("a.b(c,d)").unwrap().1;
        // assert_eq!(function_call.name, "b");
        // let function_call = parse("a.b(c,d+e)").unwrap().1;
        // assert_eq!(function_call.name, "b");
        let function_call = parse("gpio.write(s.reduce())").unwrap().1;
        assert_eq!(function_call.name, "write");
    }
}
