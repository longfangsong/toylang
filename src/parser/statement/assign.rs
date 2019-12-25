use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::IResult;
use nom::sequence::tuple;

use crate::parser::expression::{lvalue, rvalue, variable};
use crate::parser::expression::lvalue::LValue;
use crate::parser::expression::rvalue::RValue;
use crate::parser::statement::Statement;

#[derive(Debug)]
struct Assign<'a> {
    lhs: Box<dyn 'a + LValue>,
    rhs: Box<dyn 'a + RValue>,
}

impl Statement for Assign<'_> {
    fn generate_ssa(&self) -> String {
        let (rhs_ssa_str, rhs_value_reg_id) = self.rhs.generate_rvalue_ssa();
        let (lhs_ssa_str, lhs_reference_reg_id) = self.lhs.generate_lvalue_ssa();
        let self_ssa = format!("*%{} = %{};", lhs_reference_reg_id, rhs_value_reg_id);
        rhs_ssa_str + "\n" + &lhs_ssa_str[..] + "\n" + &self_ssa
    }
}

fn parse(code: &str) -> IResult<&str, Assign> {
    map(tuple((
        lvalue::lift(variable::parse),
        space0,
        tag("="),
        space0,
        rvalue::parse,
        tag(";")
    )), |(lhs, _, _, _, rhs, _)| Assign { lhs, rhs })(code)
}

#[test]
fn test_parse() {
    let result = parse("a= b+2+d;");
    assert_eq!(format!("{:?}", result.unwrap().1), "Assign { lhs: Variable { name: \"a\" }, rhs: BinOp { lhs: Variable { name: \"b\" }, op: Add, rhs: BinOp { lhs: Constant { value: 2 }, op: Add, rhs: Variable { name: \"d\" } } } }");
}

#[test]
fn test_ssa() {
    let expected_ssa = "%0 = b;\n%1 = 2;\n%2 = d;\n%3 = add %1, %2;\n%4 = add %0, %3;\n%5 = &a;\n*%5 = %4;";
    let result = parse("a = b+2+d;");
    assert_eq!(result.unwrap().1.generate_ssa(), expected_ssa);
}