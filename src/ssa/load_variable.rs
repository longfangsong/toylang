use std::fmt::{Display, Error, Formatter};

use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, space0};
use nom::combinator::map;
use nom::IResult;
use nom::lib::std::collections::hash_map::RandomState;
use nom::lib::std::collections::HashMap;
use nom::sequence::tuple;

use crate::register::{PhysicalRegister, SSARegister};
use crate::ssa::SSAStatement;

#[derive(Debug)]
pub(crate) struct LoadVariable<'a> {
    pub(crate) to: SSARegister,
    pub(crate) variable_name: &'a str,
}

impl Display for LoadVariable<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "{} = {};", self.to, self.variable_name)
    }
}

impl SSAStatement for LoadVariable<'_> {
    fn require_registers(&self) -> Vec<SSARegister> {
        vec![self.to]
    }

    fn generate_asm(&self, register_map: &HashMap<SSARegister, PhysicalRegister, RandomState>) -> String {
        let to_reg = register_map.get(&self.to).unwrap();
        let real_reg = to_reg.real_reg_or("t0");
        format!("la t1, .{}\nlw {}, 0(t1)\n", self.variable_name, real_reg) + &to_reg.store_reg_code("t0")[..]
    }
}

impl<'a> LoadVariable<'a> {
    fn parse(ir: &'a str) -> IResult<&'a str, Self> {
        map(tuple((SSARegister::parse, space0, tag("="), space0, alphanumeric1, tag(";"))),
            |(to, _, _, _, variable_name, _)| {
                LoadVariable {
                    to,
                    variable_name,
                }
            })(ir)
    }
}

#[test]
fn test_parse() {
    let result = LoadVariable::parse("%0 = a;");
    assert_eq!(result.as_ref().unwrap().1.to, SSARegister(0));
    assert_eq!(result.as_ref().unwrap().1.variable_name, "a");
}

#[test]
fn test_require_registers() {
    let result = LoadVariable::parse("%0 = a;").unwrap().1;
    assert_eq!(result.require_registers().len(), 1);
    assert_eq!(result.require_registers()[0], SSARegister(0));
}

#[test]
fn test_generate_asm() {
    let result = LoadVariable::parse("%0 = a;").unwrap().1;
    let mut register_map = HashMap::new();
    register_map.insert(SSARegister(0), PhysicalRegister::RealRegister("t3"));
    let asm = result.generate_asm(&register_map);
    assert_eq!(asm, "la t1, .a\nlw t3, 0(t1)\n");

    register_map.insert(SSARegister(0), PhysicalRegister::SpilledRegister(4));
    let asm = result.generate_asm(&register_map);
    assert_eq!(asm, "la t1, .a\nlw t0, 0(t1)\nsw t0, -4(fp)\n");
}