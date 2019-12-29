use std::collections::BTreeSet;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0};
use nom::combinator::map;
use nom::IResult;
use nom::lib::std::collections::BTreeMap;
use nom::sequence::tuple;

use crate::code_generator::{CodeGenerator, register};
use crate::code_generator::register::{make_sure_in_real_reg, put_real_reg_back, Register};

#[derive(Debug, Clone)]
pub(crate) struct LoadInstant {
    to: register::Register,
    instant: u32,
}

impl<'a> CodeGenerator<'a> for LoadInstant {
    fn generate_asm(&self) -> String {
        let (_, real_reg) = make_sure_in_real_reg(&self.to, "t0");
        format!("li {}, {}\n", real_reg, self.instant) + &put_real_reg_back(&self.to, "t0")[..]
    }

    fn using_regs(&self) -> BTreeSet<&Register> {
        let mut result = BTreeSet::new();
        result.insert(&self.to);
        result
    }

    fn assign_regs(&self, dict: &BTreeMap<&Register, Register>) -> Box<dyn CodeGenerator<'a>> {
        Box::new(LoadInstant {
            instant: self.instant,
            to: dict.get(&self.to).unwrap().clone(),
        })
    }
}

pub(crate) fn parse(ir: &str) -> IResult<&str, LoadInstant> {
    map(tuple((register::abstract_register::parse,
               space0, tag("="), space0, digit1, tag(";"))),
        |(to, _, _, _, instant_str, _)| {
            let instant = u32::from_str(instant_str).unwrap();
            LoadInstant {
                to: register::Register::AbstractRegister(to),
                instant,
            }
        })(ir)
}

#[test]
fn test_parse() {
    let result = parse("%0 = 1;");
    assert_eq!(result.as_ref().unwrap().1.to, register::Register::AbstractRegister(0));
    assert_eq!(result.as_ref().unwrap().1.instant, 1);
}

#[test]
fn test_generate_asm() {
    let mut result = parse("%0 = 1;");
    result.as_mut().unwrap().1.to = register::Register::PhysicalRegister("t3");
    assert_eq!("li t3, 1\n", result.as_ref().unwrap().1.generate_asm());
    result.as_mut().unwrap().1.to = register::Register::Memory(12);
    assert_eq!("li t0, 1\nsw t0, -12(fp)\n", result.as_ref().unwrap().1.generate_asm());
}