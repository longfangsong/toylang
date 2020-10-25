use sum_type::sum_type;

mod basic_block;
mod function;
mod global_definition;
mod integer_literal;
mod statements;
mod struct_literal;
mod type_definition;
mod utils;
pub mod visitor;

use crate::ir::{
    function::FunctionDefinitionVisitor, global_definition::GlobalDefinitionVisitor,
    type_definition::TypeDefinitionVisitor,
};
use function::FunctionDefinition;
use global_definition::GlobalDefinition;
use nom::character::complete::multispace0;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::{branch::alt, combinator::map, IResult};
use type_definition::TypeDefinition;

sum_type! {
    pub enum IR {
        TypeDefinition,
        FunctionDefinition,
        GlobalDefinition,
    }
}

pub fn parse(code: &str) -> IResult<&str, IR> {
    alt((
        map(type_definition::parse, IR::TypeDefinition),
        map(function::parse, IR::FunctionDefinition),
        map(global_definition::parse, IR::GlobalDefinition),
    ))(code)
}

pub fn from_source(source: &str) -> IResult<&str, Vec<IR>> {
    many0(delimited(multispace0, parse, multispace0))(source)
}
