use nom::character::complete::{alpha1, alphanumeric0};
use nom::combinator::{map, recognize};
use nom::IResult;
use nom::sequence::pair;

use crate::code::expression::ExpressionResult;
use crate::code::expression::lvalue::LValue;
use crate::code::expression::rvalue::RValue;
use crate::register::SSARegister;
use crate::ssa::load_address::LoadAddress;
use crate::ssa::load_variable::LoadVariable;
use crate::tools::id_generator::next_id;

#[derive(Debug)]
pub(crate) struct Variable<'a> {
    name: &'a str
}

impl<'a> RValue<'a> for Variable<'a> {
    fn generate_ir(&self) -> ExpressionResult<'a> {
        let id = next_id();
        ExpressionResult {
            ssa_generated: vec![
                Box::new(LoadVariable {
                    to: SSARegister(id),
                    variable_name: self.name,
                })
            ],
            result: SSARegister(id),
        }
    }
}

impl<'a> LValue<'a> for Variable<'a> {
    fn generate_ir(&self) -> ExpressionResult<'a> {
        let reg = SSARegister(next_id());
        ExpressionResult {
            ssa_generated: vec![Box::new(LoadAddress {
                to: reg,
                from: self.name,
            })],
            result: reg,
        }
    }
}

impl<'a> Variable<'a> {
    pub(crate) fn parse(code: &'a str) -> IResult<&'a str, Self> {
        map(recognize(pair(alpha1, alphanumeric0)), |name| {
            Variable { name }
        })(code)
    }
}

#[test]
fn test_parse() {
    let result = Variable::parse("abcd");
    assert_eq!(result.as_ref().unwrap().1.name, "abcd");
    let result = Variable::parse("a1b2c3d4");
    assert_eq!(result.as_ref().unwrap().1.name, "a1b2c3d4");
    let result = Variable::parse("a2 d");
    assert_eq!(result.as_ref().unwrap().1.name, "a2");
    assert_eq!(result.as_ref().unwrap().0, " d");
    let result = Variable::parse("2a d");
    assert!(result.is_err());
}

#[test]
fn test_generate_lvalue_ir() {
    use crate::tools::id_generator::reset_id;
    let result = Variable::parse("abcd").unwrap().1;
    reset_id();
    let ir = LValue::generate_ir(&result);
    assert_eq!(ir.result, SSARegister(0));
    assert_eq!("%0 = &abcd;\n", format!("{}", &ir.ssa_generated[0]));
}
