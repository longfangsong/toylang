use std::fmt::{Error, Formatter};
use std::fmt::Display;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0};
use nom::combinator::map;
use nom::IResult;
use nom::lib::std::collections::HashMap;
use nom::sequence::tuple;

use crate::register::{PhysicalRegister, SSARegister};
use crate::ssa::SSAStatement;

#[derive(Debug)]
pub(crate) struct LoadInstant {
    pub(crate) to: SSARegister,
    pub(crate) instant: u32,
}

impl Display for LoadInstant {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "{} = {};", self.to, self.instant)
    }
}

impl SSAStatement for LoadInstant {
    fn require_registers(&self) -> Vec<SSARegister> {
        vec![self.to]
    }

    fn generate_asm(&self, register_map: &HashMap<SSARegister, PhysicalRegister>) -> String {
        let to_reg = register_map.get(&self.to).unwrap();
        let real_reg = to_reg.real_reg_or("t0");
        format!("li {}, {}\n", real_reg, self.instant) + &to_reg.store_reg_code("t0")[..]
    }
}

impl LoadInstant {
    pub(crate) fn parse(ir: &str) -> IResult<&str, Self> {
        map(tuple((SSARegister::parse,
                   space0, tag("="), space0, digit1, tag(";"))),
            |(to, _, _, _, instant_str, _)| {
                let instant = u32::from_str(instant_str).unwrap();
                LoadInstant {
                    to,
                    instant,
                }
            })(ir)
    }
}

#[test]
fn test_parse() {
    let result = LoadInstant::parse("%0 = 1;");
    assert_eq!(result.as_ref().unwrap().1.to, SSARegister(0));
    assert_eq!(result.as_ref().unwrap().1.instant, 1);
}

#[test]
fn test_require_registers() {
    let result = LoadInstant::parse("%0 = 1;").unwrap().1;
    assert_eq!(result.require_registers().len(), 1);
    assert_eq!(result.require_registers()[0], SSARegister(0));
}

#[test]
fn test_generate_asm() {
    let result = LoadInstant::parse("%0 = 1;").unwrap().1;
    let mut register_map = HashMap::new();
    register_map.insert(SSARegister(0), PhysicalRegister::RealRegister("t3"));
    let asm = result.generate_asm(&register_map);
    assert_eq!(asm, "li t3, 1\n");

    register_map.insert(SSARegister(0), PhysicalRegister::SpilledRegister(4));
    let asm = result.generate_asm(&register_map);
    assert_eq!(asm, "li t0, 1\nsw t0, -4(fp)\n");
}