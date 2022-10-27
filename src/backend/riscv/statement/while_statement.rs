use crate::{
    ast::statement::while_statement::While,
    backend::riscv::{compile_compound, rvalue::compile_rvalue, FunctionCompileContext},
};

pub fn compile_while_statement(ctx: &mut FunctionCompileContext, statement: &While) -> String {
    let While { condition, content } = statement;
    let id = ctx.compile_context.next_while_id;
    ctx.compile_context.next_while_id += 1;
    let start_label = format!("label_while_{}_start", id);
    let condition_false_label = format!("label_while_{}_false", id);
    let (condition_register, condition_code) = compile_rvalue(ctx, condition);
    let code_for_true = compile_compound(ctx, content);
    format!(
        r#"{}:
{}
beqz {}, {}
{}
j {}
{}:"#,
        start_label,
        condition_code,
        condition_register,
        condition_false_label,
        code_for_true,
        start_label,
        condition_false_label
    )
}
