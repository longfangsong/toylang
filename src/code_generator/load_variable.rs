use std::collections::BTreeSet;

use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, space0};
use nom::combinator::map;
use nom::IResult;
use nom::lib::std::collections::BTreeMap;
use nom::sequence::tuple;

use crate::code_generator::{CodeGenerator, register};
use crate::code_generator::register::{make_sure_in_real_reg, put_real_reg_back, Register};

#[derive(Debug, Clone)]
pub(crate) struct LoadVariable<'a> {
    to: register::Register,
    variable_name: &'a str,
}

// fixme: cannot use .{} directly!
// 分两步读
impl<'a> CodeGenerator<'a> for LoadVariable<'a> {
    fn generate_asm(&self) -> String {
        let (_, real_reg_name) = make_sure_in_real_reg(&self.to, "t0");
        format!("la t1, .{}\nlw {}, 0(t1)\n", self.variable_name, real_reg_name) + &put_real_reg_back(&self.to, "t0")[..]
    }

    fn using_regs(&self) -> BTreeSet<&Register> {
        let mut result = BTreeSet::new();
        result.insert(&self.to);
        result
    }

    fn assign_regs(&self, dict: &BTreeMap<&Register, Register>) -> Box<dyn CodeGenerator<'a> + 'a> {
        Box::new(LoadVariable {
            variable_name: self.variable_name,
            to: dict.get(&self.to).unwrap().clone(),
        })
    }
}

pub(crate) fn parse(ir: &str) -> IResult<&str, LoadVariable> {
    map(tuple((register::abstract_register::parse, space0, tag("="), space0, alphanumeric1, tag(";"))),
        |(to, _, _, _, variable_name, _)| {
            LoadVariable {
                to: register::Register::AbstractRegister(to),
                variable_name,
            }
        })(ir)
}

#[test]
fn test_parse() {
    let result = parse("%4 = a;").unwrap().1;
    assert_eq!(result.variable_name, "a");
    assert_eq!(result.to, register::Register::AbstractRegister(4));
}

#[test]
fn test_generate_asm() {
    let mut result = parse("%4 = a;").unwrap().1;
    result.to = register::Register::PhysicalRegister("t3");
    assert_eq!(result.generate_asm(), "la t1, .a\nlw t3, 0(t1)\n");
    result.to = register::Register::Memory(4);
    assert_eq!(result.generate_asm(), "la t1, .a\nlw t0, 0(t1)\nsw t0, -4(fp)\n");
}