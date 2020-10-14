use crate::ast::context::Context;
use crate::ast::expression::rvalue::RValue;
use crate::ast::expression::variable_ref::VariableRef;
use crate::ast::expression::{rvalue, variable_ref, ExpressionResult};
use crate::ir::store::StoreTarget;
use crate::ir::{RegisterRef, Store, IR};
use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Assign {
    lhs: VariableRef,
    rhs: RValue,
}

pub fn parse(code: &str) -> IResult<&str, Assign> {
    map(
        tuple((
            variable_ref::parse,
            space0,
            tag("="),
            space0,
            rvalue::parse,
            space0,
            tag(";"),
        )),
        |(lhs, _, _, _, rhs, _, _)| Assign { lhs, rhs },
    )(code)
}

impl Assign {
    pub fn ir(&self) -> Vec<IR> {
        let mut result: ExpressionResult = self.rhs.ir();
        match result {
            ExpressionResult::Constant(n) => vec![Store {
                source: n.into(),
                target: StoreTarget::Global(self.lhs.0.clone()),
            }
            .into()],
            ExpressionResult::Complex {
                result,
                mut ir_generated,
            } => {
                let result: RegisterRef = (&result).into();
                ir_generated.push(
                    Store {
                        source: result.into(),
                        target: StoreTarget::Global(self.lhs.0.clone()),
                    }
                    .into(),
                );
                ir_generated
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::context::CONTEXT;
    use crate::ast::expression::bin_op::BinOp;
    use crate::ir::calculate::CalculateOperation;
    use crate::ir::store::StoreSource;
    use crate::ir::Calculate;
    use crate::shared::data_type::Integer;
    use std::convert::TryInto;
    use sum_type::SumType;

    #[test]
    fn it_works() {
        CONTEXT.insert_variable(
            "a",
            Integer {
                signed: true,
                width: 32,
            },
        );
        CONTEXT.insert_variable(
            "b",
            Integer {
                signed: true,
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
        let assign = parse("a = 1+2*3;").unwrap().1;
        assert_eq!(assign.lhs, VariableRef("a".to_string()));
        let rhs: BinOp = assign.rhs.try_into().unwrap();
        assert_eq!(rhs.operator, "+");
        let assign = parse("a = b+c-d;").unwrap().1;
        let ir = assign.ir();
        let op1: Calculate = ir[2].clone().try_into().unwrap();
        assert_eq!(op1.operation, CalculateOperation::Add);
        let op1: Calculate = ir[4].clone().try_into().unwrap();
        assert_eq!(op1.operation, CalculateOperation::Sub);
        let assign = parse("a = b+(c-d);").unwrap().1;
        let ir = assign.ir();
        let op1: Calculate = ir[3].clone().try_into().unwrap();
        assert_eq!(op1.operation, CalculateOperation::Sub);
        let op1: Calculate = ir[4].clone().try_into().unwrap();
        assert_eq!(op1.operation, CalculateOperation::Add);

        let assign = parse("a = 0;").unwrap().1;
        let ir = assign.ir();
        assert_eq!(ir.len(), 1);
        let ir: Store = ir[0].clone().try_into().unwrap();
        assert_eq!(ir.source, StoreSource::NumberLiteral(0));
    }
}
