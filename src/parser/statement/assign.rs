use crate::ir::store::StoreTarget;
use crate::ir::{Store, IR};
use crate::parser::context::Context;
use crate::parser::expression::rvalue::RValue;
use crate::parser::expression::variable::Variable;
use crate::parser::expression::{rvalue, variable, ExpressionResult};
use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Assign {
    lhs: Variable,
    rhs: RValue,
}

pub fn parse(code: &str) -> IResult<&str, Assign> {
    map(
        tuple((
            variable::parse,
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
            ExpressionResult::Constant(_) => unimplemented!(),
            ExpressionResult::Complex {
                result,
                mut ir_generated,
            } => {
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
    use crate::ir::calculate::CalculateOperation;
    use crate::ir::Calculate;
    use crate::parser::context::CONTEXT;
    use crate::parser::expression::bin_op::BinOp;
    use std::convert::TryInto;
    use sum_type::SumType;

    #[test]
    fn it_works() {
        let assign = parse("a = 1+2*3;").unwrap().1;
        assert_eq!(assign.lhs, Variable("a".to_string()));
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
    }
}
