use crate::ir::utils::{local, Local};
use crate::shared::data_type;
use crate::shared::data_type::Type;
use nom::bytes::complete::tag;
use nom::character::complete::{space0, space1};
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Alloca {
    pub to: Local,
    pub alloc_type: Type,
}

impl Display for Alloca {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} = alloca {}", self.to, self.alloc_type)
    }
}

pub fn parse(code: &str) -> IResult<&str, Alloca> {
    map(
        tuple((
            local::parse,
            space0,
            tag("="),
            space0,
            tag("alloca"),
            space1,
            data_type::parse,
        )),
        |(to_register, _, _, _, _, _, alloc_type)| Alloca {
            to: to_register,
            alloc_type,
        },
    )(code)
}
