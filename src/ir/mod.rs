use enum_dispatch::enum_dispatch;

mod basic_block;
mod function;
mod global_definition;
mod integer_literal;
mod statements;
mod struct_literal;
mod symbol_table;
mod type_definition;
mod utils;

pub use crate::ir::type_definition::TypeDefinitionVisitor;
pub use basic_block::BasicBlock;
pub use function::FunctionDefinition;
pub use global_definition::GlobalDefinition;
use nom::{
    branch::alt, character::complete::multispace0, combinator::map, multi::many0,
    sequence::delimited, IResult,
};
pub use type_definition::TypeDefinition;

#[enum_dispatch]
pub enum IR {
    TypeDefinition,
    FunctionDefinition,
    GlobalDefinition,
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
