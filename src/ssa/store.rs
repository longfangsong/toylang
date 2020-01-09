use std::collections::HashMap;
use std::fmt::Display;

use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::IResult;
use nom::lib::std::fmt::{Error, Formatter};
use nom::sequence::tuple;

use crate::register::{PhysicalRegister, SSARegister};
use crate::ssa::SSAStatement;

#[derive(Debug, Clone)]
pub(crate) struct Store {
    pub(crate) to: SSARegister,
    pub(crate) from: SSARegister,
}

impl Display for Store {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "*{} = {};", self.to, self.from)
    }
}

impl SSAStatement for Store {
    fn require_registers(&self) -> Vec<SSARegister> {
        vec![self.from, self.to]
    }

    fn generate_asm(&self, register_map: &HashMap<SSARegister, PhysicalRegister>) -> String {
        let from_reg = register_map.get(&self.from).unwrap();
        let to_reg = register_map.get(&self.to).unwrap();
        let real_from_reg = from_reg.real_reg_or("t0");
        let real_to_reg = to_reg.real_reg_or("t1");
        from_reg.load_reg_code("t0") +
            &to_reg.load_reg_code("t1") +
            &format!("sw {}, 0({})\n", real_from_reg, real_to_reg)
    }
}

impl Store {
    pub(crate) fn parse(ir: &str) -> IResult<&str, Self> {
        map(tuple((tag("*"), SSARegister::parse, space0, tag("="), space0, SSARegister::parse, tag(";"))),
            |(_, to, _, _, _, from, _)| {
                Store {
                    from,
                    to,
                }
            })(ir)
    }
}

#[test]
fn test_parse() {
    let result = Store::parse("*%1 = %0;");
    assert_eq!(result.as_ref().unwrap().1.from, SSARegister(0));
    assert_eq!(result.as_ref().unwrap().1.to, SSARegister(1));
    let result = Store::parse("%1 = a;");
    assert!(result.is_err());
}

#[test]
fn test_require_registers() {
    let statement = Store::parse("*%1 = %0;").unwrap().1;
    assert_eq!(statement.require_registers().len(), 2);
}

#[test]
fn test_generate_asm() {
    let statement = Store::parse("*%1 = %0;").unwrap().1;
    let mut register_map = HashMap::new();
    register_map.insert(SSARegister(0), PhysicalRegister::RealRegister("t3"));
    register_map.insert(SSARegister(1), PhysicalRegister::RealRegister("t4"));
    let ssa = statement.generate_asm(&register_map);
    assert_eq!(ssa, "sw t3, 0(t4)\n");
    register_map.insert(SSARegister(1), PhysicalRegister::SpilledRegister(4));
    let ssa = statement.generate_asm(&register_map);
    assert_eq!(ssa, "lw t1, -4(fp)\nsw t3, 0(t1)\n");
    register_map.insert(SSARegister(0), PhysicalRegister::SpilledRegister(8));
    let ssa = statement.generate_asm(&register_map);
    assert_eq!(ssa, "lw t0, -8(fp)\nlw t1, -4(fp)\nsw t0, 0(t1)\n");
}