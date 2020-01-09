use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};

use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::combinator::map;
use nom::IResult;
use nom::sequence::{pair, tuple};

use crate::register::{PhysicalRegister, SSARegister};
use crate::ssa::SSAStatement;

pub(crate) struct Label {
    pub(crate) name: String
}

impl Display for Label {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "{}:", self.name)
    }
}

impl SSAStatement for Label {
    fn require_registers(&self) -> Vec<SSARegister> {
        vec![]
    }

    fn generate_asm(&self, register_map: &HashMap<SSARegister, PhysicalRegister>) -> String {
        self.name.clone() + ":\n"
    }
}

impl Label {
    pub(crate) fn parse(ir: &str) -> IResult<&str, Label> {
        map(pair(alphanumeric1, tag(":")), |(name, _): (&str, _)| {
            Label { name: name.to_string() }
        })(ir)
    }
}

#[test]
fn test_parse() {
    let result = Label::parse("asdf:").unwrap();
    assert_eq!(result.1.name, "asdf");
    let result = Label::parse("%0 = sth %1, %2;");
    assert!(result.is_err());
}