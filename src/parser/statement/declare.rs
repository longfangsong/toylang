use crate::ir::{Alloca, IR};
use crate::ir::{Global, Register as LogicalRegister};
use crate::parser::context::CONTEXT;
use crate::parser::expression::variable_ref;
use crate::parser::expression::variable_ref::VariableRef;
use crate::shared::data_type;
use crate::shared::data_type::Integer;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric0, space0, space1};
use nom::combinator::{map, recognize};
use nom::sequence::{pair, tuple};
use nom::IResult;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Declare {
    pub variable_name: String,
    pub data_type: Integer,
}

pub fn parse(code: &str) -> IResult<&str, Declare> {
    map(
        tuple((
            tag("let"),
            space1,
            map(recognize(pair(alpha1, alphanumeric0)), |x: &str| {
                x.to_string()
            }),
            space0,
            tag(":"),
            space0,
            data_type::parse_integer,
            space0,
            tag(";"),
        )),
        |(_, _, variable_name, _, _, _, data_type, _, _)| Declare {
            variable_name,
            data_type,
        },
    )(code)
}

impl Declare {
    pub fn ir(&self) -> IR {
        let name = self.variable_name.clone();
        CONTEXT.insert_variable(&name, self.data_type.clone());
        Global {
            name,
            data_type: self.data_type.clone(),
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
        let code = parse("let a: i32;").unwrap().1;
        assert_eq!(code.variable_name, "a");
        assert_eq!(
            code.data_type,
            Integer {
                signed: true,
                width: 32,
            }
        );

        let ir: Global = code.ir().try_into().unwrap();
        assert_eq!(
            ir,
            Global {
                name: "a".to_string(),
                data_type: Integer {
                    signed: true,
                    width: 32,
                },
                initial_value: 0,
            }
        );
    }
}
