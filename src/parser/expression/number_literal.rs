use std::str::FromStr;

use nom::branch::alt;
use nom::character::complete::digit1;
use nom::combinator::{map, map_res, recognize};
use nom::IResult;
use nom::number::complete::double;
use num::{Float, Integer, Num};
use typename::TypeName;

use crate::parser::Context;
use crate::parser::expression::result::ExpressionParseResult;
use crate::parser::expression::rvalue::RValue;
use crate::symbol_table::table::SymbolTable;
use crate::util::lift::lift;

impl<T: Num + TypeName + ToString> RValue for T {
    fn generate_rvalue(&self) -> ExpressionParseResult {
        ExpressionParseResult {
            type_name: self.type_name_of(),
            generated_code: "".to_string(),
            bind_to: self.to_string(),
        }
    }
}

pub fn integer_literal<'a, T: Num + FromStr>(input: (&'a str, Context<'a>)) -> IResult<(&'a str, Context<'a>), T> {
    map_res(lift(digit1), |s: &str| s.parse::<T>())(input)
}

pub fn float_literal<'a, T: Num + FromStr>(input: (&'a str, Context<'a>)) -> IResult<(&'a str, Context<'a>), T> {
    map_res(lift(recognize(double)), |s: &str| s.parse::<T>())(input)
}

pub fn number_literal<'a, T: Num + FromStr + RValue>(input: (&'a str, Context<'a>)) -> IResult<(&'a str, Context<'a>), T> {
    alt((integer_literal, float_literal))(input)
}