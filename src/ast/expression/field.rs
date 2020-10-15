use crate::ast::expression::rvalue::RValue;
use crate::ast::expression::{parenthesis, variable_ref};
use crate::shared::parse;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, not, peek};
use nom::multi::many1;
use nom::sequence::tuple;
use nom::IResult;
use std::convert::TryInto;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Field {
    from: Box<RValue>,
    name: String,
}

pub fn higher_than_field(code: &str) -> IResult<&str, RValue> {
    alt((
        map(variable_ref::parse, RValue::VariableRef),
        map(parenthesis::parse, RValue::Parenthesis),
    ))(code)
}

fn to_ast(first: RValue, rest: Vec<String>) -> Field {
    let mut current_lhs = first;
    for field_name in rest.into_iter() {
        current_lhs = Field {
            from: Box::new(current_lhs),
            name: field_name,
        }
        .into();
    }
    current_lhs.try_into().unwrap()
}

pub fn parse(code: &str) -> IResult<&str, Field> {
    map(
        tuple((
            higher_than_field,
            many1(map(
                tuple((tag("."), parse::ident, peek(not(tag("("))))),
                |(_, name, _)| name,
            )),
        )),
        |(from, names)| to_ast(from, names),
    )(code)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::expression::rvalue::parse as rvalue_parse;
    use std::convert::TryInto;

    #[test]
    fn can_parse() {
        let field = rvalue_parse("a.b").unwrap().1;
        let field: Field = field.try_into().unwrap();
        assert_eq!(field.name, "b");
        let field = rvalue_parse("a.b.c").unwrap().1;
        let field: Field = field.try_into().unwrap();
        assert_eq!(field.name, "c");
        let field = rvalue_parse("b.c()").unwrap().1;
        let field: Result<Field, _> = field.try_into();
        assert!(field.is_err());
    }
}
