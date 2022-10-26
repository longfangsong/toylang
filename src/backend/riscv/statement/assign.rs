use crate::{
    ast::{expression::lvalue::LValue, statement::assign::Assign},
    backend::riscv::{rvalue::compile_rvalue, FunctionCompileContext},
};

pub fn compile_assign(ctx: &mut FunctionCompileContext, assign: &Assign) -> String {
    if let LValue::VariableRef(variable_name) = &assign.lhs {
        let lhs_offset = *ctx.local_stack_offsets.get(&variable_name.0).unwrap();
        let (rhs_register, rhs_code) = compile_rvalue(ctx, &assign.rhs);
        format!("{}\nsw {}, {}(sp)", rhs_code, rhs_register, lhs_offset)
    } else {
        unimplemented!()
    }
}
