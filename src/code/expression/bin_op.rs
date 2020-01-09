use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::Err;
use nom::error::ErrorKind;
use nom::IResult;
use nom::sequence::tuple;

use crate::code::expression::{ExpressionResult, rvalue};
use crate::code::expression::constant::Constant;
use crate::code::expression::rvalue::RValue;
use crate::code::expression::variable::Variable;
use crate::register::SSARegister;
use crate::ssa;
use crate::tools::id_generator::next_id;

lazy_static! {
    pub(crate) static ref OP_NAME: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("+", "add");
        m.insert("-", "sub");
        m.insert("<", "slt");
        m
    };
}

pub struct BinOp<'a> {
    lhs: Box<dyn RValue<'a> + 'a>,
    op: &'static str,
    rhs: Box<dyn RValue<'a> + 'a>,
}

impl<'a> RValue<'a> for BinOp<'a> {
    fn generate_ir(&self) -> ExpressionResult<'a> {
        let lhs_ir = self.lhs.generate_ir();
        let rhs_ir = self.rhs.generate_ir();
        let id = next_id();
        let self_ssa_generated = Box::new(ssa::bin_op::BinOp {
            result: SSARegister(id),
            lhs: lhs_ir.result,
            op: OP_NAME.get(self.op).unwrap(),
            rhs: rhs_ir.result,
        });
        let mut ssa_generated = lhs_ir.ssa_generated;
        ssa_generated.extend(rhs_ir.ssa_generated);
        ssa_generated.push(self_ssa_generated);
        ExpressionResult {
            result: SSARegister(id),
            ssa_generated,
        }
    }
}

fn parse_op(code: &str) -> IResult<&str, &'static str> {
    for &op in OP_NAME.keys() {
        let this_match: IResult<&str, &str> = tag(op)(code);
        if this_match.is_ok() {
            return Ok((this_match.unwrap().0, op));
        }
    }
    Err(Err::Error(("No op found", ErrorKind::Alt)))
}

#[test]
fn test_parse_op() {
    let result = parse_op("+ 2").unwrap();
    assert_eq!(result.1, "+");
    assert_eq!(result.0, " 2");
    let result = parse_op("asdf");
    assert!(result.is_err());
    let result = parse_op("1234");
    assert!(result.is_err());
    let result = parse_op(" +");
    assert!(result.is_err());
}

impl<'a> BinOp<'a> {
    pub fn parse(code: &'a str) -> IResult<&'a str, BinOp<'a>> {
        let constant_lifted = rvalue::lift(Constant::parse);
        let variable_lifted = rvalue::lift(Variable::parse);
        map(tuple((
            alt((constant_lifted, variable_lifted)),
            space0,
            parse_op,
            space0,
            rvalue::parse
        )), |(lhs, _, op, _, rhs)| {
            BinOp { lhs, op, rhs }
        })(code)
    }
}

#[test]
fn test_parse() {
    let result = BinOp::parse("1234 + 5678");
    assert!(result.is_ok());

    let result = BinOp::parse("a + 2 + c");
    assert!(result.is_ok());
}

#[test]
fn test_generate_ir() {
    use crate::tools::id_generator::reset_id;
    let result = BinOp::parse("1234 + 5678").unwrap().1;
    reset_id();
    let result = result.generate_ir();
    assert_eq!(result.result, SSARegister(2));
    assert_eq!(result.ssa_generated.len(), 3);
    assert_eq!(result.ssa_generated[0].require_registers().len(), 1);
}
