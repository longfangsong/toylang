use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, space0};
use nom::combinator::{map, opt};
use nom::IResult;
use nom::sequence::tuple;

use crate::parser::Context;
use crate::parser::expression::rvalue::rvalue;
use crate::parser::expression::variable_reference::variable_reference;
use crate::parser::statement::assignment::Assignment;
use crate::parser::statement::Statement;
use crate::symbol_table::symbol::Symbol;

pub struct VariableDeclaration {
    pub variable: Symbol,
    pub init: Option<Assignment>,
}

impl Statement for VariableDeclaration {
    fn generate_code(self) -> String {
        let code = format!("{} = alloca {}", self.variable.declaration_name(), self.variable.type_name);
        let init_assignment = self.init.map_or_else("".to_string(), |it| it.generate_code() + "\n");
        code + "\n" + init_assignment.as_str()
    }
}

fn type_name<'a>(input: (&'a str, Context<'a>)) -> IResult<(&'a str, Context<'a>), &str> {
    alt((
        tag("i1"),
        tag("i8"),
        tag("i16"),
        tag("i32"),
        tag("i64"),
        tag("u1"),
        tag("u8"),
        tag("u16"),
        tag("u32"),
        tag("u64"),
    ))(input)
}

pub fn variable_declaration<'a>(input: (&'a str, Context<'a>)) -> IResult<(&'a str, Context<'a>), VariableDeclaration> {
    map(tuple((
        type_name,
        space0,
        variable_reference,
        opt(tuple((
            space0,
            char('='),
            rvalue
        )))
    )), |(type_name, _, variable, assignment)| {
        let assign_value = assignment.map(|(_, _, rvalue)| Assignment {
            lhs: Box::new(variable),
            rhs: Box::new(rvalue),
        });
        VariableDeclaration {
            variable,
            init: assign_value,
        }
    })
}