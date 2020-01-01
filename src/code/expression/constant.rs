use nom::character::complete::digit1;
use nom::combinator::map;
use nom::IResult;

use crate::code::expression::ExpressionResult;
use crate::code::expression::rvalue::RValue;
use crate::register::SSARegister;
use crate::ssa::load_instant::LoadInstant;
use crate::tools::id_generator::next_id;

#[derive(Debug)]
pub(crate) struct Constant {
    value: u32
}

impl<'a> RValue<'a> for Constant {
    fn generate_ir(&self) -> ExpressionResult<'a> {
        let id = next_id();
        let reg = SSARegister(id);
        ExpressionResult {
            ssa_generated: vec![Box::new(LoadInstant {
                to: reg,
                instant: self.value,
            })],
            result: reg,
        }
    }
}

impl Constant {
    pub(crate) fn parse(code: &str) -> IResult<&str, Self> {
        map(digit1,
            |digits| Constant { value: u32::from_str_radix(digits, 10).unwrap() },
        )(code)
    }
}

#[test]
fn test_parse() {
    let result = Constant::parse("1234");
    assert_eq!(result.as_ref().unwrap().1.value, 1234);
    let result = Constant::parse("asdf");
    assert!(result.is_err());
    let result = Constant::parse("0 asdf");
    assert_eq!(result.as_ref().unwrap().1.value, 0);
    assert_eq!(result.as_ref().unwrap().0, " asdf");
}

#[test]
fn test_generate_ir() {
    use crate::tools::id_generator::reset_id;
    reset_id();
    let result = Constant::parse("1234").unwrap().1;
    let ir = result.generate_ir();
    assert_eq!(ir.result, SSARegister(0));
    assert_eq!(ir.ssa_generated.len(), 1);
    assert_eq!(format!("{}", ir.ssa_generated[0]), "%0 = 1234;\n");
}