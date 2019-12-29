use std::collections::BTreeSet;

use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, space0};
use nom::combinator::map;
use nom::IResult;
use nom::lib::std::collections::BTreeMap;
use nom::sequence::tuple;

use crate::code_generator::{CodeGenerator, register};
use crate::code_generator::register::Register;

#[derive(Debug, Clone)]
pub(crate) struct LoadAddress<'a> {
    from: &'a str,
    to: register::Register,
}

impl<'a> CodeGenerator<'a> for LoadAddress<'a> {
    fn generate_asm(&self) -> String {
        match self.to {
            register::Register::PhysicalRegister(to) => format!("la {}, .{}", to, self.from),
            _ => panic!("LoadAddress's result must be putted into a real register!")
        }
    }

    fn using_regs(&self) -> BTreeSet<&Register> {
        let mut result = BTreeSet::new();
        result.insert(&self.to);
        result
    }

    fn assign_regs(&self, dict: &BTreeMap<&Register, Register>) -> Box<dyn CodeGenerator<'a> + 'a> {
        Box::new(LoadAddress {
            from: self.from,
            to: dict.get(&self.to).unwrap().clone(),
        })
    }
}

pub(crate) fn parse(ir: &str) -> IResult<&str, LoadAddress> {
    map(tuple((register::abstract_register::parse, space0, tag("="), space0, tag("&"), alphanumeric1, tag(";"))),
        |(to, _, _, _, _, from, _)| {
            let to = Register::AbstractRegister(to);
            LoadAddress {
                from,
                to,
            }
        })(ir)
}

#[test]
fn test_parse() {
    let result = parse("%1 = &a;");
    assert_eq!(result.as_ref().unwrap().1.from, "a");
    assert_eq!(result.as_ref().unwrap().1.to, Register::AbstractRegister(1));
    let result = parse("%1 = a;");
    assert!(result.is_err());
}

#[test]
fn test_generate_asm() {
    let mut result = parse("%1 = &a;");
    result.as_mut().unwrap().1.to = Register::PhysicalRegister("t1");
    assert_eq!(result.unwrap().1.generate_asm(), "la t1, .a");
}