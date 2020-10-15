use sum_type::sum_type;

pub mod assign;
pub mod compound;
pub mod declare;
pub mod if_statement;
pub mod return_statement;
pub mod while_statement;

use crate::ast::statement::assign::AssignVisitor;
use crate::ast::statement::declare::DeclareVisitor;
use crate::ast::statement::if_statement::{If, IfVisitor};
use crate::ast::statement::return_statement::{Return, ReturnVisitor};
use crate::ast::statement::while_statement::{While, WhileVisitor};
use assign::Assign;
use declare::Declare;
use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;

sum_type! {
    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum Statement {
        Declare,
        Assign,
        Return,
        If,
        While,
    }
}

pub fn parse(code: &str) -> IResult<&str, Statement> {
    alt((
        map(declare::parse, Statement::Declare),
        map(assign::parse, Statement::Assign),
        map(return_statement::parse, Statement::Return),
        map(if_statement::parse, Statement::If),
        map(while_statement::parse, Statement::While),
    ))(code)
}

pub trait StatementVisitor:
    DeclareVisitor + AssignVisitor + ReturnVisitor + IfVisitor + WhileVisitor
{
    fn visit_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Declare(declare) => self.visit_declare(declare),
            Statement::Assign(assign) => self.visit_assign(assign),
            Statement::Return(return_statement) => self.visit_return(return_statement),
            Statement::If(if_statement) => self.visit_if(if_statement),
            Statement::While(while_statement) => self.visit_while(while_statement),
        }
    }
}
