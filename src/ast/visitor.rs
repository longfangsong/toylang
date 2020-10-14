use crate::ast::expression::rvalue::RValue;
use crate::ast::statement::Statement;

pub trait ASTVisitor {
    fn visit_statement(&mut self, statement: &Statement);
    fn visit_expression(&mut self, expression: &RValue);
}
