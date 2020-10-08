use crate::ir::register::{parse as parse_register, Register};
use crate::shared::data_type;
use crate::shared::data_type::Integer;
use nom::bytes::complete::tag;
use nom::character::complete::{space0, space1};
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Alloca {
    pub to: Register,
    pub data_type: Integer,
}

impl Display for Alloca {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} = alloca", self.to)
    }
}

pub fn parse(code: &str) -> IResult<&str, Alloca> {
    map(
        tuple((
            parse_register,
            space0,
            tag("="),
            space0,
            tag("alloca"),
            space1,
            data_type::parse,
        )),
        |(to, _, _, _, _, _, data_type)| Alloca { to, data_type },
    )(code)
}

impl Alloca {
    pub fn create_register(&self) -> &Register {
        &self.to
    }
}
