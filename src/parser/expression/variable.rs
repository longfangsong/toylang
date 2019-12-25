use nom::character::complete::{alpha1, alphanumeric0};
use nom::combinator::{map, recognize};
use nom::IResult;
use nom::sequence::pair;

use crate::parser::expression::lvalue::LValue;
use crate::parser::expression::rvalue::RValue;
use crate::tools::id_generator::next_id;

#[derive(Debug)]
pub struct Variable<'a> {
    name: &'a str
}

pub fn parse(code: &str) -> IResult<&str, Variable> {
    map(recognize(pair(alpha1, alphanumeric0)), |name| {
        Variable { name }
    })(code)
}

impl RValue for Variable<'_> {
    fn generate_rvalue_ssa(&self) -> (String, u64) {
        let id = next_id();
        return (format!("%{} = {};", id, self.name), id);
    }
}

impl LValue for Variable<'_> {
    fn generate_lvalue_ssa(&self) -> (String, u64) {
        let id = next_id();
        (format!("%{} = &{};", id, self.name), id)
    }
}

#[test]
fn test_parse() {
    let result = parse("abcd");
    assert_eq!(result.as_ref().unwrap().1.name, "abcd");
    let result = parse("a1b2c3d4");
    assert_eq!(result.as_ref().unwrap().1.name, "a1b2c3d4");
    let result = parse("a2 d");
    assert_eq!(result.as_ref().unwrap().1.name, "a2");
    assert_eq!(result.as_ref().unwrap().0, " d");
    let result = parse("2a d");
    assert!(result.is_err());
}