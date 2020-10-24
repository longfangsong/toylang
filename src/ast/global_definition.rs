use crate::ast::statement::{
    declare,
    declare::{Declare, DeclareVisitor},
};
use nom::{combinator::map, IResult};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct VariableDefinition(Declare);

pub trait VariableDefinitionVisitor: DeclareVisitor {
    fn visit_global_definition(&mut self, variable_definition: &VariableDefinition);
}

pub fn parse(code: &str) -> IResult<&str, VariableDefinition> {
    map(declare::parse, VariableDefinition)(code)
}
