use std::fmt::{self, Display, Formatter};
use nom::IResult;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, space0, space1};
use crate::RegisterCreator;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Alloca {
    pub to_register: String,
    pub data_type: String,
}

impl Display for Alloca {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "%{} = alloca {}", self.to_register, self.data_type)
    }
}

pub fn alloca(code: &str) -> IResult<&str, Alloca> {
    map(tuple((tag("%"), alphanumeric1, space0, tag("="), space0, tag("alloca"), space1, alphanumeric1)),
        |(_, to_register, _, _, _, _, _, data_type): (_, &str, _, _, _, _, _, _)| Alloca { to_register: to_register.to_string(), data_type: data_type.to_string() },
    )(code)
}

impl RegisterCreator for Alloca {
    fn created(&self) -> &str {
        &self.to_register
    }
}