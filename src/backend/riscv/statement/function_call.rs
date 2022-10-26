use crate::{
    ast::expression::{self, function_call::FunctionCall},
    backend::riscv::FunctionCompileContext,
};

pub fn compile_function_call(
    ctx: &mut FunctionCompileContext,
    function_call_statement: &FunctionCall,
) -> String {
    let expression::function_call::FunctionCall { name, arguments } = function_call_statement;
    let mut result = String::new();
    let push_stack_count = usize::max(arguments.len(), ctx.parameter_registers.len());
    for id in 0..push_stack_count {
        result += format!("sw a{}, ({})sp", id, ctx.next_stack_offset).as_str();
        ctx.next_stack_offset += 4;
        ctx.stack_space_used = usize::max(ctx.stack_space_used, ctx.next_stack_offset);
    }
    result += format!("call {}", name).as_str();
    for id in 0..push_stack_count {
        ctx.next_stack_offset -= 4;
        result += format!("lw a{}, ({})sp", id, ctx.next_stack_offset).as_str();
    }
    result
}
