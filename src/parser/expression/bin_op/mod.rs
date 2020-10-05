// levels are same with C's Operator Precedence
// todo: replace these with macros
mod level10;
mod level3;
mod level4;
mod level5;
mod level6;
mod level7;
mod level8;
mod level9;

use crate::ir::calculate::{CalculateOperation, Operand};
use crate::ir::{Calculate, Store, IR};
use crate::parser::context::CONTEXT;
use crate::parser::expression::constant::Constant;
use crate::parser::expression::rvalue::RValue;
use crate::parser::expression::ExpressionResult;
use nom::branch::alt;
use nom::IResult;
use std::collections::HashMap;
use std::convert::TryInto;

lazy_static! {
    static ref OPERATION_MAP: HashMap<String, CalculateOperation> = {
        let mut result = HashMap::new();
        result.insert("+".to_string(), CalculateOperation::Add);
        result.insert("-".to_string(), CalculateOperation::Sub);
        result.insert("<".to_string(), CalculateOperation::Less);
        result.insert("|".to_string(), CalculateOperation::Or);
        result.insert("&".to_string(), CalculateOperation::And);
        result.insert("^".to_string(), CalculateOperation::Xor);
        result.insert(">>".to_string(), CalculateOperation::SRL);
        result.insert("<<".to_string(), CalculateOperation::SLL);
        result
    };
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct BinOp {
    pub operator: String,
    pub lhs: Box<RValue>,
    pub rhs: Box<RValue>,
}

fn to_ast(first: RValue, rest: Vec<(String, RValue)>) -> BinOp {
    let mut current_lhs = first;
    for (op, value) in rest.into_iter() {
        current_lhs = BinOp {
            operator: op,
            lhs: Box::new(current_lhs),
            rhs: Box::new(value),
        }
        .into()
    }
    current_lhs.try_into().unwrap()
}

pub fn parse(code: &str) -> IResult<&str, BinOp> {
    alt((
        level10::parse,
        level9::parse,
        level8::parse,
        level7::parse,
        level6::parse,
        level5::parse,
        level4::parse,
        level3::parse,
    ))(code)
}

impl BinOp {
    pub fn ir(&self) -> ExpressionResult {
        let lhs_result = self.lhs.ir();
        let rhs_result = self.rhs.ir();
        let mut ir_generated: Vec<_> = match &lhs_result {
            ExpressionResult::Constant(_) => vec![],
            ExpressionResult::Complex { ir_generated, .. } => ir_generated.clone(),
        };
        ir_generated.extend(match &rhs_result {
            ExpressionResult::Constant(_) => vec![],
            ExpressionResult::Complex { ir_generated, .. } => ir_generated.clone(),
        });
        let to_register = CONTEXT.next();
        ir_generated.push(
            Calculate {
                operation: OPERATION_MAP.get(&self.operator).unwrap().clone(),
                operand1: lhs_result.into(),
                operand2: rhs_result.into(),
                to_register: to_register.clone(),
            }
            .into(),
        );
        ExpressionResult::Complex {
            ir_generated,
            result: to_register,
        }
    }
}
