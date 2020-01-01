use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::fmt::Display;

use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, space0};
use nom::combinator::map;
use nom::IResult;
use nom::lib::std::fmt::{Error, Formatter};
use nom::sequence::tuple;

use crate::register::{PhysicalRegister, SSARegister};
use crate::ssa::SSAStatement;

#[derive(Debug)]
pub(crate) struct LoadAddress<'a> {
    pub(crate) to: SSARegister,
    pub(crate) from: &'a str,
}

impl Display for LoadAddress<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "{} = &{};", self.to, self.from)
    }
}

impl SSAStatement for LoadAddress<'_> {
    fn require_registers(&self) -> Vec<SSARegister> {
        vec![self.to]
    }

    fn generate_asm(&self, register_map: &HashMap<SSARegister, PhysicalRegister, RandomState>) -> String {
        let to_reg =
            register_map.get(&self.to).unwrap();
        let real_to = to_reg.real_reg_or("t0");
        format!("la {}, .{}\n", real_to, self.from) + &to_reg.store_reg_code("t0")
    }
}

impl LoadAddress<'_> {
    pub(crate) fn parse(ir: &str) -> IResult<&str, LoadAddress> {
        map(tuple((SSARegister::parse, space0, tag("="), space0, tag("&"), alphanumeric1, tag(";"))),
            |(to, _, _, _, _, from, _)| {
                LoadAddress {
                    from,
                    to,
                }
            })(ir)
    }
}

#[test]
fn test_parse() {
    let result = LoadAddress::parse("%1 = &a;");
    assert_eq!(result.as_ref().unwrap().1.from, "a");
    assert_eq!(result.as_ref().unwrap().1.to, SSARegister(1));
    let result = LoadAddress::parse("%1 = a;");
    assert!(result.is_err());
}

#[test]
fn test_generate_asm() {
    let mut result = LoadAddress::parse("%1 = &a;").unwrap().1;
    let mut register_map = HashMap::new();
    register_map.insert(SSARegister(1), PhysicalRegister::RealRegister("t3"));
    let asm_generated = result.generate_asm(&register_map);
    assert_eq!(asm_generated, "la t3, .a\n");

    register_map.insert(SSARegister(1), PhysicalRegister::SpilledRegister(4));
    let asm_generated = result.generate_asm(&register_map);
    assert_eq!(asm_generated, "la t0, .a\nsw t0, -4(fp)\n");
}