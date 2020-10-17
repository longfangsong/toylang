use crate::ast::expression::rvalue;
use crate::ast::expression::rvalue::RValue;
use crate::shared::data_type;
use crate::shared::data_type::Type;
use crate::shared::parsing;
use nom::bytes::complete::tag;
use nom::character::complete::{space0, space1};
use nom::combinator::{map, opt};
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Declare {
    pub variable_name: String,
    pub data_type: Type,
    pub init_value: Option<RValue>,
}

pub fn parse(code: &str) -> IResult<&str, Declare> {
    map(
        tuple((
            tag("let"),
            space1,
            parsing::ident,
            space0,
            tag(":"),
            space0,
            data_type::parse,
            space0,
            opt(map(
                tuple((space0, tag("="), space0, rvalue::parse, space0)),
                |(_, _, _, x, _)| x,
            )),
            tag(";"),
        )),
        |(_, _, variable_name, _, _, _, data_type, _, init_value, _)| Declare {
            variable_name,
            data_type,
            init_value,
        },
    )(code)
}

pub trait DeclareVisitor {
    fn visit_declare(&mut self, declare: &Declare);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse() {
        let declare = parse("let gpio: address = 0x40000000;").unwrap().1;
        assert_eq!(declare.variable_name, "gpio");
        let declare = parse("let s:S = S { a: 1, b: 2 };").unwrap().1;
        assert_eq!(declare.variable_name, "s");
    }
}
