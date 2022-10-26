use crate::{
    ast::{
        expression::{lvalue::LValue, variable_ref::VariableRef},
        statement::{assign::Assign, declare::Declare},
    },
    backend::riscv::FunctionCompileContext,
};

use super::assign::compile_assign;
pub fn compile_declare(ctx: &mut FunctionCompileContext, declare: &Declare) -> String {
    let Declare {
        variable_name,
        data_type: _,
        init_value,
    } = declare;
    let stack_offset = ctx.next_stack_offset;
    ctx.local_stack_offsets
        .insert(variable_name.clone(), stack_offset);
    ctx.next_stack_offset += 4;
    ctx.stack_space_used = usize::max(ctx.stack_space_used, ctx.next_stack_offset);
    if let Some(init_value) = init_value {
        compile_assign(
            ctx,
            &Assign {
                lhs: LValue::VariableRef(VariableRef(variable_name.clone())),
                rhs: init_value.clone(),
            },
        )
    } else {
        String::new()
    }
}
