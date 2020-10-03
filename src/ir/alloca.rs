use crate::ir::register::{register, Register};
use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Alloca {
    pub to: Register,
    // todo: pub data_type: String,
}

impl Display for Alloca {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} = alloca", self.to)
    }
}

pub fn alloca(code: &str) -> IResult<&str, Alloca> {
    map(
        tuple((register, space0, tag("="), space0, tag("alloca"))),
        |(to, _, _, _, _)| Alloca { to },
    )(code)
}

impl Alloca {
    pub fn create_register(&self) -> &Register {
        &self.to
    }
}
