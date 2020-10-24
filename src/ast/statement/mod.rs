use sum_type::sum_type;

pub mod assign;
pub mod compound;
pub mod declare;
pub mod function_call;
pub mod if_statement;
pub mod return_statement;
pub mod while_statement;

use assign::{Assign, AssignVisitor};
use declare::{Declare, DeclareVisitor};
use function_call::{FunctionCall, FunctionCallVisitor};
use if_statement::{If, IfVisitor};
use nom::{branch::alt, combinator::map, IResult};
use return_statement::{Return, ReturnVisitor};
use while_statement::{While, WhileVisitor};

sum_type! {
    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum Statement {
        Declare,
        Assign,
        Return,
        If,
        While,
        FunctionCall,
    }
}

pub fn parse(code: &str) -> IResult<&str, Statement> {
    alt((
        map(declare::parse, Statement::Declare),
        map(assign::parse, Statement::Assign),
        map(return_statement::parse, Statement::Return),
        map(if_statement::parse, Statement::If),
        map(while_statement::parse, Statement::While),
        map(function_call::parse, Statement::FunctionCall),
    ))(code)
}

pub trait StatementVisitor:
    DeclareVisitor + AssignVisitor + ReturnVisitor + IfVisitor + WhileVisitor + FunctionCallVisitor
{
    fn visit_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Declare(declare) => self.visit_declare(declare),
            Statement::Assign(assign) => self.visit_assign(assign),
            Statement::Return(return_statement) => self.visit_return(return_statement),
            Statement::If(if_statement) => self.visit_if(if_statement),
            Statement::While(while_statement) => self.visit_while(while_statement),
            Statement::FunctionCall(function_call) => self.visit_function_call(function_call),
        }
    }
}
