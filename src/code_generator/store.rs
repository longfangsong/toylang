use std::collections::BTreeSet;

use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::IResult;
use nom::lib::std::collections::BTreeMap;
use nom::sequence::tuple;

use crate::code_generator::{CodeGenerator, register};
use crate::code_generator::register::{make_sure_in_real_reg, Register};

#[derive(Debug, Clone)]
pub(crate) struct Store {
    to: register::Register,
    from: register::Register,
}

impl<'a> CodeGenerator<'a> for Store {
    fn generate_asm(&self) -> String {
        let (from_code, from_real_reg) = make_sure_in_real_reg(&self.from, "t0");
        let (to_code, to_real_reg) = make_sure_in_real_reg(&self.to, "t1");
        from_code + &to_code[..] + &format!("sw {}, 0({})", from_real_reg, to_real_reg)[..]
    }

    fn using_regs(&self) -> BTreeSet<&Register> {
        let mut result = BTreeSet::new();
        result.insert(&self.from);
        result.insert(&self.to);
        result
    }

    fn assign_regs(&self, dict: &BTreeMap<&Register, Register>) -> Box<dyn CodeGenerator<'a>> {
        Box::new(Store {
            from: dict.get(&self.from).unwrap().clone(),
            to: dict.get(&self.to).unwrap().clone(),
        })
    }
}

pub(crate) fn parse(ir: &str) -> IResult<&str, Store> {
    map(tuple((tag("*"), register::abstract_register::parse, space0, tag("="), space0, register::abstract_register::parse, tag(";"))),
        |(_, to, _, _, _, from, _)| {
            let from = Register::AbstractRegister(from);
            let to = Register::AbstractRegister(to);
            Store {
                from,
                to,
            }
        })(ir)
}

#[test]
fn test_parse() {
    let result = parse("*%1 = %0;");
    assert_eq!(result.as_ref().unwrap().1.from, Register::AbstractRegister(0));
    assert_eq!(result.as_ref().unwrap().1.to, Register::AbstractRegister(1));
    let result = parse("%1 = a;");
    assert!(result.is_err());
}

#[test]
fn test_generate_asm() {
    let mut result = parse("*%1 = %0;").unwrap().1;
    result.to = Register::PhysicalRegister("t3");
    result.from = Register::PhysicalRegister("t3");
    assert_eq!(result.generate_asm(), "sw t3, 0(t3)");

    let mut result = parse("*%1 = %0;").unwrap().1;
    result.to = Register::PhysicalRegister("t3");
    result.from = Register::Memory(12);
    assert_eq!(result.generate_asm(), "lw t0, -12(fp)\nsw t0, 0(t3)");

    let mut result = parse("*%1 = %0;").unwrap().1;
    result.to = Register::Memory(12);
    result.from = Register::PhysicalRegister("t3");
    assert_eq!(result.generate_asm(), "lw t1, -12(fp)\nsw t3, 0(t1)");

    let mut result = parse("*%1 = %0;").unwrap().1;
    result.to = Register::Memory(12);
    result.from = Register::Memory(16);
    assert_eq!(result.generate_asm(), "lw t0, -16(fp)\nlw t1, -12(fp)\nsw t0, 0(t1)");
}