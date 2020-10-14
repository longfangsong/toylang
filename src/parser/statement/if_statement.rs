use crate::ir::branch::BranchType;
use crate::ir::calculate::CalculateOperation;
use crate::ir::{Branch, Jump, Label, IR};
use crate::parser::context::CONTEXT;
use crate::parser::expression::bin_op::OPERATION_MAP;
use crate::parser::expression::rvalue::RValue;
use crate::parser::expression::{rvalue, ExpressionResult};
use crate::parser::statement::compound;
use crate::parser::statement::compound::Compound;
use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;
use std::borrow::Borrow;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct If {
    condition: RValue,
    content: Compound,
}

fn generate_branch(condition: &RValue) -> (Vec<IR>, String) {
    match condition {
        RValue::Parenthesis(x) => generate_branch(x.0.borrow()),
        RValue::BinOp(x) => {
            let condition_op = OPERATION_MAP.get(&x.operator).unwrap();
            let lhs_result = x.lhs.ir();
            let (lhs_ir, lhs_result) = if let ExpressionResult::Complex {
                ir_generated,
                result,
            } = lhs_result
            {
                (ir_generated, result)
            } else {
                unimplemented!()
            };
            let rhs_result = x.rhs.ir();
            let (rhs_ir, rhs_result) = if let ExpressionResult::Complex {
                ir_generated,
                result,
            } = rhs_result
            {
                (ir_generated, result)
            } else {
                unimplemented!()
            };
            let next_branch_id = CONTEXT.next_branch_id();
            let branch = match condition_op {
                CalculateOperation::EQ => Branch {
                    branch_type: BranchType::EQ,
                    operand1: (&lhs_result).into(),
                    operand2: (&rhs_result).into(),
                    success_label: next_branch_id.clone() + "_true",
                    failure_label: next_branch_id.clone() + "_end",
                },
                CalculateOperation::NE => Branch {
                    branch_type: BranchType::NE,
                    operand1: (&lhs_result).into(),
                    operand2: (&rhs_result).into(),
                    success_label: next_branch_id.clone() + "_true",
                    failure_label: next_branch_id.clone() + "_end",
                },
                CalculateOperation::LT => Branch {
                    branch_type: BranchType::LT,
                    operand1: (&lhs_result).into(),
                    operand2: (&rhs_result).into(),
                    success_label: next_branch_id.clone() + "_true",
                    failure_label: next_branch_id.clone() + "_end",
                },
                CalculateOperation::GE => Branch {
                    branch_type: BranchType::GE,
                    operand1: (&lhs_result).into(),
                    operand2: (&rhs_result).into(),
                    success_label: next_branch_id.clone() + "_true",
                    failure_label: next_branch_id.clone() + "_end",
                },
                CalculateOperation::LE => Branch {
                    branch_type: BranchType::GE,
                    operand1: (&rhs_result).into(),
                    operand2: (&lhs_result).into(),
                    success_label: next_branch_id.clone() + "_true",
                    failure_label: next_branch_id.clone() + "_end",
                },
                CalculateOperation::GT => Branch {
                    branch_type: BranchType::LT,
                    operand1: (&rhs_result).into(),
                    operand2: (&lhs_result).into(),
                    success_label: next_branch_id.clone() + "_true",
                    failure_label: next_branch_id.clone() + "_end",
                },
                _ => unreachable!(),
            };
            let mut ir = lhs_ir;
            ir.extend(rhs_ir.into_iter());
            ir.push(branch.into());
            (ir, next_branch_id)
        }
        _ => unreachable!(),
    }
}

pub fn parse(code: &str) -> IResult<&str, If> {
    map(
        tuple((tag("if"), space0, rvalue::parse, space0, compound::parse)),
        |(_, _, condition, _, content)| If { condition, content },
    )(code)
}

impl If {
    pub fn ir(&self) -> Vec<IR> {
        let (mut result, label) = generate_branch(&self.condition);
        result.push(Label(label.clone() + "_true").into());
        result.extend(self.content.ir());
        result.push(Label(label + "_end").into());
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::data_type::Integer;

    #[test]
    fn it_works() {
        CONTEXT.insert_variable(
            "a",
            Integer {
                signed: false,
                width: 32,
            },
        );
        CONTEXT.insert_variable(
            "b",
            Integer {
                signed: false,
                width: 32,
            },
        );
        CONTEXT.insert_variable(
            "c",
            Integer {
                signed: true,
                width: 32,
            },
        );
        CONTEXT.insert_variable(
            "d",
            Integer {
                signed: true,
                width: 32,
            },
        );
        let item = parse("if a > b { c = d; }").unwrap().1;
        let ir = item.ir();
        for ir in ir {
            println!("{}", ir);
        }
    }
}
