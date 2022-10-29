use std::collections::HashMap;

use crate::{
    ast::{function_definition::FunctionDefinition, statement::compound::Compound, Ast},
    utility::data_type::{Integer, Type},
};
mod rvalue;
mod statement;
pub struct FunctionCompileContext<'a> {
    compile_context: &'a mut CompileContext,
    parameter_registers: HashMap<String, usize>,
    next_temporary_register: usize,
    next_stack_offset: usize,
    stack_space_used: usize,
    local_stack_offsets: HashMap<String, usize>,
}

pub fn compile_function(ctx: &mut CompileContext, function: &FunctionDefinition) -> String {
    let FunctionDefinition {
        name,
        parameters,
        return_type: _,
        content,
    } = function;
    let mut result = format!("{}:", name);
    let mut context = FunctionCompileContext {
        parameter_registers: HashMap::new(),
        compile_context: ctx,
        next_temporary_register: 0,
        next_stack_offset: 0,
        local_stack_offsets: HashMap::new(),
        stack_space_used: 0,
    };
    for (next_register, parameter) in parameters.iter().enumerate() {
        if parameter.data_type
            != Type::Integer(Integer {
                signed: true,
                width: 32,
            })
        {
            unimplemented!("Only i32 parameters are supported now");
        }
        context
            .parameter_registers
            .insert(parameter.name.clone(), next_register);
    }
    let body_result = compile_compound(&mut context, content);
    result += format!("\naddi sp, sp, -{}\n", context.stack_space_used).as_str();
    result += body_result.as_str();
    result += format!("\naddi sp, sp, {}\n", context.stack_space_used).as_str();
    result += "ret\n";
    result
}

pub fn compile_compound(ctx: &mut FunctionCompileContext, compound: &Compound) -> String {
    compound
        .0
        .iter()
        .map(|statement| statement::compile_statement(ctx, statement))
        .collect::<Vec<_>>()
        .join("\n")
}

pub struct CompileContext {
    next_while_id: usize,
    next_if_id: usize,
}

pub fn compile(ast: &Ast) -> String {
    let mut ctx = CompileContext {
        next_while_id: 0,
        next_if_id: 0,
    };
    let mut result = String::new();
    for node in ast {
        result += match node {
            crate::ast::ASTNode::TypeDefinition(_) => todo!(),
            crate::ast::ASTNode::FunctionDefinition(function) => {
                compile_function(&mut ctx, function)
            }
            crate::ast::ASTNode::GlobalVariableDefinition(_) => todo!(),
        }
        .as_str()
    }
    result
}
