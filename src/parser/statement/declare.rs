use crate::ir::{Alloca, IR};
use crate::ir::{Global, Register as LogicalRegister};
use crate::parser::context::Context;
use crate::parser::expression::variable;
use crate::parser::expression::variable::Variable;
use nom::bytes::complete::tag;
use nom::character::complete::{space0, space1};
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Declare(pub Variable);

pub fn parse(code: &str) -> IResult<&str, Declare> {
    map(
        tuple((tag("let"), space1, variable::parse, space0, tag(";"))),
        |(_, _, variable, _, _)| Declare(variable),
    )(code)
}

impl Declare {
    pub fn ir(&self) -> IR {
        let name = (self.0).0.clone();
        Global {
            name,
            initial_value: 0,
        }
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn it_works() {
        let code = parse("let a;").unwrap().1;
        assert_eq!(code.0.0,"a");
        let ir: Global = code.ir().try_into().unwrap();
        assert_eq!(
            ir,
            Global {
                name: "a".to_string(),
                initial_value: 0
            }
        );
    }
}
