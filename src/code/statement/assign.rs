use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::IResult;
use nom::sequence::tuple;

use crate::code::expression::{lvalue, rvalue};
use crate::code::expression::lvalue::LValue;
use crate::code::expression::rvalue::RValue;
use crate::code::statement::Statement;
use crate::ssa::SSAStatement;
use crate::ssa::store::Store;

pub(crate) struct Assign<'a> {
    lhs: Box<dyn LValue<'a> + 'a>,
    rhs: Box<dyn RValue<'a> + 'a>,
}

impl<'a> Statement<'a> for Assign<'a> {
    fn generate_ir(&self) -> Vec<Box<dyn SSAStatement + 'a>> {
        let lhs_ir = self.lhs.generate_ir();
        let rhs_ir = self.rhs.generate_ir();
        let self_statement = Store {
            to: lhs_ir.result,
            from: rhs_ir.result,
        };
        let mut result = lhs_ir.ssa_generated;
        result.extend(rhs_ir.ssa_generated);
        result.push(Box::new(self_statement));
        result
    }
}

impl Assign<'_> {
    pub(crate) fn parse(code: &str) -> IResult<&str, Assign> {
        map(tuple((
            lvalue::parse,
            space0,
            tag("="),
            space0,
            rvalue::parse,
            tag(";")
        )), |(lhs, _, _, _, rhs, _)| Assign { lhs, rhs })(code)
    }
}

#[test]
fn test_parse() {
    let result = Assign::parse("a= b+2+d;");
    assert!(result.is_ok());
}

#[test]
fn test_generate_ir() {
    let result = Assign::parse("a=b+2+d;").unwrap().1;
    let generated = result.generate_ir();
    assert_eq!(result.generate_ir().len(), 7);
}