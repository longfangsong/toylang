use nom::character::complete::digit1;
use nom::combinator::map;
use nom::IResult;

use crate::parser::expression::rvalue::RValue;
use crate::tools::id_generator::next_id;

#[derive(Debug)]
pub struct Constant {
    value: u32
}

impl RValue for Constant {
    fn generate_rvalue_ir(&self) -> (String, u64) {
        let id = next_id();
        return (format!("%{} = {};", id, self.value), id);
    }
}

pub fn parse(code: &str) -> IResult<&str, Constant> {
    map(digit1,
        |s| Constant { value: u32::from_str_radix(s, 10).unwrap() },
    )(code)
}

#[test]
fn test_parse() {
    let result = parse("1234");
    assert_eq!(result.as_ref().unwrap().1.value, 1234);
    let result = parse("asdf");
    assert!(result.is_err());
    let result = parse("0 asdf");
    assert_eq!(result.as_ref().unwrap().1.value, 0);
    assert_eq!(result.as_ref().unwrap().0, " asdf");
}