use crate::{
    ast::statement::return_statement::Return,
    backend::riscv::{rvalue::compile_rvalue, FunctionCompileContext},
};

pub fn compile_return_statement(
    ctx: &mut FunctionCompileContext,
    return_statement: &Return,
) -> String {
    if let Some(return_value) = &return_statement.0 {
        let (rhs_register, rhs_code) = compile_rvalue(ctx, return_value);
        format!("{}\nmv a0, {}", rhs_code, rhs_register)
    } else {
        "ret".to_string()
    }
}
