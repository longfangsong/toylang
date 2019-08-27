use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric0, alphanumeric1, char};
use nom::combinator::{map, map_res, recognize};
use nom::error::VerboseError;
use nom::IResult;
use nom::multi::many0;
use nom::sequence::tuple;

use crate::parser::Context;
use crate::parser::expression::lvalue::LValue;
use crate::parser::expression::result::ExpressionParseResult;
use crate::parser::expression::rvalue::RValue;
use crate::symbol_table::symbol::Symbol;
use crate::util::lift::lift;
use crate::util::sequence::SEQUENCE;

pub struct VariableReference {
    pub variable: Symbol,
}

impl RValue for VariableReference {
    fn generate_rvalue(&self) -> ExpressionParseResult {
        let next_sequence = SEQUENCE.next();
        let type_name = self.variable.type_name.to_string();
        let declaration_name = self.variable.declaration_name();
        let temp_variable_name = format!("{}_reference{}", declaration_name, next_sequence);
        let code = format!("{} = load {}, {}* {}",
                           temp_variable_name,
                           type_name,
                           type_name,
                           declaration_name);
        ExpressionParseResult {
            type_name: type_name.to_owned(),
            generated_code: code,
            bind_to: temp_variable_name,
        }
    }
}

impl LValue for VariableReference {
    fn generate_lvalue(self) -> ExpressionParseResult {
        ExpressionParseResult {
            type_name: self.variable.type_name.clone(),
            generated_code: "".to_string(),
            bind_to: self.variable.declaration_name(),
        }
    }
}

pub fn variable_reference<'a>(input: (&'a str, Context<'a>)) -> IResult<(&'a str, Context<'a>), VariableReference> {
    map(lift(recognize::<&str, _, _, _>(tuple((
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    )))), |name_result| VariableReference {
        variable: input.1.symbol_table.find_entry(name_result).expect("faq")
    })(input)
}