use crate::ast::function::FunctionDefinition;
use crate::ast::global_definition::VariableDefinition;
use crate::ast::type_definition::TypeDefinition;
use nom::branch::alt;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::IResult;
use sum_type::sum_type;

mod expression;
mod function;
mod global_definition;
mod statement;
mod type_definition;
pub(crate) mod visitor;

sum_type! {
    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum ASTNode {
        Type(TypeDefinition),
        Function(FunctionDefinition),
        GlobalVariable(VariableDefinition),
    }
}

pub fn parse(code: &str) -> IResult<&str, ASTNode> {
    alt((
        map(type_definition::parse, ASTNode::Type),
        map(function::parse, ASTNode::Function),
        map(global_definition::parse, ASTNode::GlobalVariable),
    ))(code)
}

pub type AST = Vec<ASTNode>;

pub fn from_source(source: &str) -> IResult<&str, AST> {
    many0(delimited(multispace0, parse, multispace0))(source)
}
