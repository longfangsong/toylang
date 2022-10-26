use enum_dispatch::enum_dispatch;
use nom::{
    branch::alt, character::complete::multispace0, combinator::map, multi::many0,
    sequence::delimited, IResult,
};

use self::{
    function::FunctionDefinition, global_definition::VariableDefinition,
    type_definition::TypeDefinition,
};

pub mod expression;
pub mod function;
pub mod global_definition;
pub mod statement;
pub mod type_definition;

#[enum_dispatch]
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum ASTNode {
    Type(TypeDefinition),
    Function(FunctionDefinition),
    GlobalVariable(VariableDefinition),
}

pub fn parse(code: &str) -> IResult<&str, ASTNode> {
    alt((
        map(type_definition::parse, ASTNode::Type),
        map(function::parse, ASTNode::Function),
        map(global_definition::parse, ASTNode::GlobalVariable),
    ))(code)
}

pub type Ast = Vec<ASTNode>;

pub fn from_source(source: &str) -> IResult<&str, Ast> {
    many0(delimited(multispace0, parse, multispace0))(source)
}
