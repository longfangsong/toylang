use nom::character::complete::{char, space0};
use nom::combinator::map;
use nom::IResult;
use nom::sequence::tuple;

use crate::parser::Context;
use crate::parser::expression::lvalue::{lvalue, LValue};
use crate::parser::expression::rvalue::{RValue, rvalue};
use crate::parser::statement::Statement;
use crate::symbol_table::symbol::Symbol;

pub struct Assignment {
    pub lhs: Box<dyn LValue>,
    pub rhs: Box<dyn RValue>,
}

impl Statement for Assignment {
    fn generate_code(self) -> String {
        let value_generated = self.rhs.generate_rvalue();
        let code = format!("store {} {}, {}* {}",
                           self.lhs.type_name,
                           self.lhs.declaration_name(),
                           self.lhs.type_name,
                           value_generated.bind_to);
        value_generated.generated_code +
            if value_generated.generated_code.len() != 0 { "\n" } else { "" } +
            code.as_str()
    }
}

pub fn variable_assignment<'a>(input: (&'a str, Context<'a>)) -> IResult<(&'a str, Context<'a>), Assignment> {
    map(tuple((
        lvalue,
        space0,
        char('='),
        space0,
        rvalue
    )), |(lhs, _, _, _, rhs)| Assignment {
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    })
}