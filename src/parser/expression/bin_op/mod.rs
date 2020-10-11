use paste::paste;
macro_rules! bin_op_level {
    ($n: expr, $n_1: expr, $($op: expr)*) => {
        paste! {
            mod [<level $n>] {
                use crate::parser::expression::bin_op::{
                    self, BinOp,
                    [<level $n_1>]::{self, [<higher_than_level $n_1>]}
                };
                use crate::parser::expression::rvalue::RValue;
                use nom::branch::alt;
                use nom::bytes::complete::tag;
                use nom::character::complete::space0;
                use nom::combinator::map;
                use nom::multi::many1;
                use nom::sequence::tuple;
                use nom::IResult;

                pub(in crate::parser::expression) fn [<higher_than_level $n>](
                    code: &str,
                ) -> IResult<&str, RValue> {
                    alt((
                        map([<level $n_1>]::parse, |binop| binop.into()),
                        [<higher_than_level $n_1>],
                    ))(code)
                }

                pub fn parse(code: &str) -> IResult<&str, BinOp> {
                    map(
                        tuple((
                            [<higher_than_level $n>],
                            many1(map(
                                tuple((
                                    space0,
                                    alt((
                                        $(
                                            tag($op),
                                        )*
                                    )),
                                    space0,
                                   [<higher_than_level $n>],
                                )),
                                |(_, op, _, operand)| (op.to_string(), operand),
                            )),
                        )),
                        |(first, rest)| bin_op::to_ast(first, rest),
                    )(code)
                }
            }
        }
    };
}

// levels are same with C's Operator Precedence
mod level3;
bin_op_level!(4, 3, "+" "-");
bin_op_level!(5, 4, "<<" ">>");
bin_op_level!(6, 5, "<=" "<" ">=" ">");
bin_op_level!(7, 6, "==" "!=");
bin_op_level!(8, 7, "&" "&");
bin_op_level!(9, 8, "^" "^");
bin_op_level!(10, 9, "|" "|");

use crate::ir::calculate::{CalculateOperation, Operand};
use crate::ir::{Calculate, Store, IR};
use crate::parser::context::CONTEXT;
use crate::parser::expression::constant::Constant;
use crate::parser::expression::rvalue::RValue;
use crate::parser::expression::ExpressionResult;
use crate::shared::data_type::{Integer, Type};
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
        let mut data_type = Integer {
            signed: true,
            width: 32,
        };
        let mut ir_generated: Vec<_> = match &lhs_result {
            ExpressionResult::Constant(_) => vec![],
            ExpressionResult::Complex {
                ir_generated,
                result,
            } => {
                if let Type::Integer(integer) = &result.data_type {
                    data_type = integer.clone().into();
                    ir_generated.clone()
                } else {
                    unreachable!()
                }
            }
        };
        ir_generated.extend(match &rhs_result {
            ExpressionResult::Constant(_) => vec![],
            ExpressionResult::Complex {
                ir_generated,
                result,
            } => {
                if let Type::Integer(integer) = &result.data_type {
                    data_type = integer.clone().into();
                    ir_generated.clone()
                } else {
                    unreachable!()
                }
            }
        });
        let to_register = CONTEXT.next(data_type);
        ir_generated.push(
            Calculate {
                operation: *OPERATION_MAP.get(&self.operator).unwrap(),
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
