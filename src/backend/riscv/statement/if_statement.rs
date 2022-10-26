use crate::{
    ast::statement::if_statement::If,
    backend::riscv::{compile_compound, rvalue::compile_rvalue, FunctionCompileContext},
};

pub fn compile_if_statement(ctx: &mut FunctionCompileContext, statement: &If) -> String {
    let If {
        condition,
        content,
        else_content,
    } = statement;
    let id = ctx.compile_context.next_if_id;
    ctx.compile_context.next_if_id += 1;
    let _condition_true_label = format!("label_if_{}_true", id);
    let condition_false_label = format!("label_if_{}_false", id);
    let (condition_register, condition_code) = compile_rvalue(ctx, condition);
    let code_for_true = compile_compound(ctx, content);
    let mut current_result = format!(
        r#"{}
beqz {}, {}
{}"#,
        condition_code, condition_register, condition_false_label, code_for_true
    );
    if let Some(else_content) = else_content {
        let if_end_label = format!("label_if_{}_end", id);
        let code_for_false = compile_compound(ctx, else_content);
        current_result += format!(
            r#"
j {}
{}:
{}
{}:"#,
            if_end_label, condition_false_label, code_for_false, if_end_label
        )
        .as_str();
    } else {
        current_result += format!("\n{}:", condition_false_label).as_str();
    }
    current_result
}
