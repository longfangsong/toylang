use sum_type::sum_type;

mod basic_block;
mod function;
mod global_definition;
mod integer_literal;
mod statements;
mod struct_literal;
mod type_definition;
mod utils;

use crate::ir::{
    function::FunctionDefinitionVisitor, global_definition::GlobalDefinitionVisitor,
    type_definition::TypeDefinitionVisitor,
};
use function::FunctionDefinition;
use global_definition::GlobalDefinition;
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

pub trait IRVisitor:
    TypeDefinitionVisitor + FunctionDefinitionVisitor + GlobalDefinitionVisitor
{
    fn visit_ir(&mut self, ir: &IR) {
        match ir {
            IR::TypeDefinition(type_definition) => self.visit_type_definition(type_definition),
            IR::FunctionDefinition(function_definition) => {
                self.visit_function_definition(function_definition)
            }
            IR::GlobalDefinition(global_definition) => {
                self.visit_global_definition(global_definition)
            }
        }
    }
}
