use crate::ast::expression::{
    binary_operator, in_brackets, integer_literal, rvalue::RValue, unary_operator, variable_ref,
};

use super::FunctionCompileContext;

pub fn compile_rvalue(ctx: &mut FunctionCompileContext, rvalue: &RValue) -> (String, String) {
    match rvalue {
        RValue::IntegerLiteral(integer_literal::IntegerLiteral(n)) => {
            let register = ctx.next_temporary_register;
            ctx.next_temporary_register += 1;
            (format!("t{}", register), format!("li t{}, {}", register, n))
        }
        RValue::VariableRef(variable_ref::VariableRef(name)) => {
            if let Some(param_index) = ctx.parameter_registers.get(name) {
                (format!("a{}", param_index), String::new())
            } else if let Some(variable_offset) = ctx.local_stack_offsets.get(name) {
                let register = ctx.next_temporary_register;
                ctx.next_temporary_register += 1;
                (
                    format!("t{}", register),
                    // todo: recycle temp register
                    format!("lw t{}, {}(sp)", register, variable_offset),
                )
            } else {
                unimplemented!()
            }
        }
        RValue::InBrackets(in_brackets::InBrackets(inner)) => compile_rvalue(ctx, inner.as_ref()),
        RValue::FieldAccess(_) => unimplemented!(),
        RValue::FunctionCall(function) => {
            let code = super::statement::function_call::compile_function_call(ctx, function);
            ("a0".to_string(), code)
        }
        RValue::UnaryOperatorResult(unary_operator_result) => {
            compile_unary_operator_result(ctx, unary_operator_result)
        }
        RValue::BinaryOperatorResult(binary_operator_result) => {
            compile_binary_operator_result(ctx, binary_operator_result)
        }
    }
}

fn compile_unary_operator_result(
    ctx: &mut FunctionCompileContext,
    unary_operator_result: &unary_operator::UnaryOperatorResult,
) -> (String, String) {
    let unary_operator::UnaryOperatorResult { operator, operand } = unary_operator_result;
    let (operand_register, operand_code) = compile_rvalue(ctx, operand.as_ref());
    let result_register = ctx.next_temporary_register;
    ctx.next_temporary_register += 1;
    let result_code = match operator.as_str() {
        "+" => format!("mv t{}, {}", result_register, operand_register),
        "-" => format!("neg t{}, {}", result_register, operand_register),
        "!" => format!("not t{}, {}", result_register, operand_register),
        "~" => format!("not t{}, {}", result_register, operand_register),
        _ => unreachable!(),
    };
    (
        format!("t{}", result_register),
        format!("{}\n{}", operand_code, result_code),
    )
}

fn compile_binary_operator_result(
    ctx: &mut FunctionCompileContext,
    binary_operator_result: &binary_operator::BinaryOperatorResult,
) -> (String, String) {
    let binary_operator::BinaryOperatorResult { operator, lhs, rhs } = binary_operator_result;
    let (left_register, left_code) = compile_rvalue(ctx, lhs.as_ref());
    let (right_register, right_code) = compile_rvalue(ctx, rhs.as_ref());
    let result_register = ctx.next_temporary_register;
    ctx.next_temporary_register += 1;
    let result_code = match operator.as_str() {
        "+" => format!(
            "add t{}, {}, {}",
            result_register, left_register, right_register
        ),
        "-" => format!(
            "sub t{}, {}, {}",
            result_register, left_register, right_register
        ),
        "==" => format!(
            "sub t{}, {}, {}\nseqz t{}, t{}",
            result_register, left_register, right_register, result_register, result_register
        ),
        "!=" => format!(
            "sub t{}, {}, {}\nsnez t{}, t{}",
            result_register, left_register, right_register, result_register, result_register
        ),
        "<" => format!(
            "slt t{}, {}, {}",
            result_register, left_register, right_register
        ),
        ">" => format!(
            "sgt t{}, {}, {}",
            result_register, left_register, right_register
        ),
        "<=" => format!(
            "sle t{}, {}, {}",
            result_register, left_register, right_register
        ),
        ">=" => format!(
            "sge t{}, {}, {}",
            result_register, left_register, right_register
        ),
        "&&" => format!(
            "and t{}, {}, {}",
            result_register, left_register, right_register
        ),
        "||" => format!(
            "or t{}, {}, {}",
            result_register, left_register, right_register
        ),
        "&" => format!(
            "and t{}, {}, {}",
            result_register, left_register, right_register
        ),
        "|" => format!(
            "or t{}, {}, {}",
            result_register, left_register, right_register
        ),
        "^" => format!(
            "xor t{}, {}, {}",
            result_register, left_register, right_register
        ),
        "<<" => format!(
            "sll t{}, {}, {}",
            result_register, left_register, right_register
        ),
        ">>" => format!(
            "srl t{}, {}, {}",
            result_register, left_register, right_register
        ),
        _ => unreachable!(),
    };
    (
        format!("t{}", result_register),
        format!("{}\n{}\n{}", left_code, right_code, result_code),
    )
}
