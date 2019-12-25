use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::IResult;
use nom::sequence::tuple;

use crate::parser::expression::{constant, rvalue, variable};
use crate::parser::expression::rvalue::RValue;
use crate::tools::id_generator::next_id;

#[derive(Debug)]
enum Op {
    Add,
}

#[derive(Debug)]
pub struct BinOp<'a> {
    lhs: Box<dyn 'a + rvalue::RValue>,
    op: Op,
    rhs: Box<dyn 'a + rvalue::RValue>,
}

impl RValue for BinOp<'_> {
    fn generate_rvalue_ssa(&self) -> (String, u64) {
        let (lhs_ssa_str, lhs_ssa_id) = self.lhs.generate_rvalue_ssa();
        let (rhs_ssa_str, rhs_ssa_id) = self.rhs.generate_rvalue_ssa();
        let id = next_id();
        let this_ssa = format!("%{} = add %{}, %{};", id, lhs_ssa_id, rhs_ssa_id);
        let result_str = lhs_ssa_str + "\n" + &rhs_ssa_str[..] + "\n" + &this_ssa[..];
        return (result_str, id);
    }
}

pub fn parse<'a>(code: &'a str) -> IResult<&'a str, BinOp> {
    let constant_lifted = rvalue::lift::<'a, _, _>(constant::parse);
    let variable_lifted = rvalue::lift::<'a, _, _>(variable::parse);
    map(tuple((
        alt((constant_lifted, variable_lifted)),
        space0,
        tag("+"),
        space0,
        rvalue::parse
    )), |(lhs, _, _, _, rhs)| {
        BinOp {
            lhs,
            op: Op::Add,
            rhs,
        }
    })(code)
}

#[test]
fn test_parse() {
    let result = parse("1234 + 5678");
    let content = format!("{:?}", result.as_ref().unwrap().1.lhs);
    assert_eq!(content, "Constant { value: 1234 }");
    let content = format!("{:?}", result.as_ref().unwrap().1.rhs);
    assert_eq!(content, "Constant { value: 5678 }");

    let result = parse("a+b");
    let content = format!("{:?}", result.as_ref().unwrap().1.lhs);
    assert_eq!(content, "Variable { name: \"a\" }");
    let content = format!("{:?}", result.as_ref().unwrap().1.rhs);
    assert_eq!(content, "Variable { name: \"b\" }");

    let result = parse("a +1+2");
    let content = format!("{:?}", result.as_ref().unwrap().1.lhs);
    assert_eq!(content, "Variable { name: \"a\" }");
    let content = format!("{:?}", result.as_ref().unwrap().1.rhs);
    assert_eq!(content, "BinOp { lhs: Constant { value: 1 }, op: Add, rhs: Constant { value: 2 } }");
}