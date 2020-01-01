use std::fmt::Display;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::IResult;
use nom::lib::std::cmp::Ordering;
use nom::lib::std::fmt::{Error, Formatter};
use nom::sequence::pair;

pub(crate) mod assign;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Hash, Eq, PartialEq)]
pub(crate) struct SSARegister(pub u64);

impl Display for SSARegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "%{}", self.0)
    }
}

impl SSARegister {
    pub(crate) fn parse(ir: &str) -> IResult<&str, SSARegister> {
        map(pair(tag("%"), digit1), |(_, digits)| {
            SSARegister(u64::from_str(digits).unwrap())
        })(ir)
    }
}

#[test]
fn test_parse_ssa() {
    let parsed_result = SSARegister::parse("%1");
    assert_eq!(parsed_result.unwrap().1, SSARegister(1));
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq, PartialOrd)]
pub(crate) enum PhysicalRegister {
    RealRegister(&'static str),
    SpilledRegister(u32),
}

impl PhysicalRegister {
    pub(crate) fn real_reg_or(&self, temp_reg: &'static str) -> PhysicalRegister {
        match self {
            PhysicalRegister::RealRegister(_) => self.clone(),
            PhysicalRegister::SpilledRegister(_) => PhysicalRegister::RealRegister(temp_reg)
        }
    }
    pub(crate) fn load_reg_code(&self, temp_reg: &'static str) -> String {
        if let PhysicalRegister::SpilledRegister(offset) = self {
            format!("lw {}, -{}(fp)\n", temp_reg, offset)
        } else {
            String::new()
        }
    }
    pub(crate) fn store_reg_code(&self, temp_reg: &'static str) -> String {
        if let PhysicalRegister::SpilledRegister(offset) = self {
            format!("sw {}, -{}(fp)\n", temp_reg, offset)
        } else {
            String::new()
        }
    }
}

impl Display for PhysicalRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            PhysicalRegister::RealRegister(r) => write!(f, "{}", r),
            PhysicalRegister::SpilledRegister(offset) => write!(f, "-{}(fp)", offset)
        }
    }
}

// "t0" is kept for code generation use
// "t1" is kept for code generation use,
// "t2" is kept for code generation use,
// "s0" is used as fp
pub(crate) const REAL_REGISTERS: [PhysicalRegister; 23] = [
    // caller saved
    PhysicalRegister::RealRegister("t3"),
    PhysicalRegister::RealRegister("t4"),
    PhysicalRegister::RealRegister("t5"),
    PhysicalRegister::RealRegister("t6"),
    PhysicalRegister::RealRegister("a0"),
    PhysicalRegister::RealRegister("a1"),
    PhysicalRegister::RealRegister("a2"),
    PhysicalRegister::RealRegister("a3"),
    PhysicalRegister::RealRegister("a4"),
    PhysicalRegister::RealRegister("a5"),
    PhysicalRegister::RealRegister("a6"),
    PhysicalRegister::RealRegister("a7"),
    // callee need to save
    PhysicalRegister::RealRegister("s1"),
    PhysicalRegister::RealRegister("s2"),
    PhysicalRegister::RealRegister("s3"),
    PhysicalRegister::RealRegister("s4"),
    PhysicalRegister::RealRegister("s5"),
    PhysicalRegister::RealRegister("s6"),
    PhysicalRegister::RealRegister("s7"),
    PhysicalRegister::RealRegister("s8"),
    PhysicalRegister::RealRegister("s9"),
    PhysicalRegister::RealRegister("s10"),
    PhysicalRegister::RealRegister("s11"),
];

impl Ord for PhysicalRegister {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (PhysicalRegister::RealRegister(_), PhysicalRegister::RealRegister(_)) => {
                let index_of =
                    |to_find: &PhysicalRegister| REAL_REGISTERS.iter().enumerate()
                        .find(|(_, reg)| *reg == to_find)
                        .map(|it| it.0).unwrap();
                index_of(self).cmp(&index_of(other))
            }
            (PhysicalRegister::RealRegister(_), PhysicalRegister::SpilledRegister(_)) =>
                Ordering::Less,
            (PhysicalRegister::SpilledRegister(_), PhysicalRegister::RealRegister(_)) =>
                Ordering::Greater,
            (PhysicalRegister::SpilledRegister(address_1), PhysicalRegister::SpilledRegister(address_2)) =>
                address_1.cmp(address_2),
        }
    }
}
