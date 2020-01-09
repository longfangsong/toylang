use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};

use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, space0, space1};
use nom::combinator::map;
use nom::IResult;
use nom::sequence::tuple;

use crate::register::{PhysicalRegister, SSARegister};
use crate::ssa::SSAStatement;

pub(crate) struct Branch {
    pub(crate) condition: SSARegister,
    pub(crate) on_true_label: String,
    pub(crate) on_false_label: String,
}

impl Display for Branch {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "br {}, {}, {};", self.condition, self.on_true_label, self.on_false_label)
    }
}

impl SSAStatement for Branch {
    fn require_registers(&self) -> Vec<SSARegister> {
        vec![]
    }

    fn generate_asm(&self, register_map: &HashMap<SSARegister, PhysicalRegister>) -> String {
        let condition_reg = register_map.get(&self.condition).unwrap();
        let real_reg = condition_reg.real_reg_or("t0");
        condition_reg.load_reg_code("t0") + &format!("bez {}, {}\n", real_reg, self.on_false_label)
    }
}

impl Branch {
    fn parse(ir: &str) -> IResult<&str, Self> {
        map(tuple((
            tag("br"), space1,
            SSARegister::parse,
            space0, tag(","), space0,
            alphanumeric1,
            space0, tag(","), space0,
            alphanumeric1, space0, tag(";")
        )), |(_, _,
                 condition,
                 _, _, _,
                 on_true_label,
                 _, _, _,
                 on_false_label, _, _)| {
            Branch {
                condition,
                on_true_label: on_true_label.to_string(),
                on_false_label: on_false_label.to_string(),
            }
        })(ir)
    }
}

#[test]
fn test_parse() {
    let result = Branch::parse("br %0, foo, bar;").unwrap().1;
    assert_eq!(result.condition, SSARegister(0));
    assert_eq!(result.on_true_label, "foo");
    assert_eq!(result.on_false_label, "bar");

    let result = Branch::parse("br a,d,s;");
    assert!(result.is_err());
}

#[test]
fn test_generate_asm() {
    let result = Branch::parse("br %0, foo, bar;").unwrap().1;
    let mut register_map = HashMap::new();
    register_map.insert(SSARegister(0), PhysicalRegister::RealRegister("t3"));
    let asm = result.generate_asm(&register_map);
    assert_eq!(asm, "bez t3, bar\n");

    register_map.insert(SSARegister(0), PhysicalRegister::SpilledRegister(4));
    let asm = result.generate_asm(&register_map);
    assert_eq!(asm, "lw t0, -4(fp)\nbez t0, bar\n");
}