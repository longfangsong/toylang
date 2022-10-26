use crate::ast::statement::Statement;
mod assign;
mod declare;
pub mod function_call;
mod if_statement;
mod return_statement;
mod while_statement;

use assign::compile_assign;
use declare::compile_declare;
use if_statement::compile_if_statement;
use return_statement::compile_return_statement;

use self::{function_call::compile_function_call, while_statement::compile_while_statement};

use super::FunctionCompileContext;

pub fn compile_statement(ctx: &mut FunctionCompileContext, statement: &Statement) -> String {
    match statement {
        Statement::Declare(declare) => compile_declare(ctx, declare),
        Statement::Assign(assign) => compile_assign(ctx, assign),
        Statement::Return(return_statement) => compile_return_statement(ctx, return_statement),
        Statement::If(if_statement) => compile_if_statement(ctx, if_statement),
        Statement::While(while_statement) => compile_while_statement(ctx, while_statement),
        Statement::FunctionCall(function_call_statement) => {
            compile_function_call(ctx, &function_call_statement.0)
        }
    }
}
