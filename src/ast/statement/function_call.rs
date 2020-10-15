use crate::ast::expression;
use crate::ast::expression::function_call;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::pair;
use nom::IResult;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FunctionCall(pub expression::function_call::FunctionCall);

pub fn parse(code: &str) -> IResult<&str, FunctionCall> {
    map(pair(function_call::parse, tag(";")), |(content, _)| {
        FunctionCall(content)
    })(code)
}

pub trait FunctionCallVisitor {
    fn visit_function_call(&mut self, function_call: &FunctionCall);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse() {
        let function_call = parse("gpio.write(s.reduce());").unwrap().1;
        assert_eq!(function_call.0.name, "write");
    }
}
