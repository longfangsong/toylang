use crate::{RegisterCreator, RegisterUser};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, space0, space1};
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum LoadSource {
    Global(String),
    Local(String),
}

impl Display for LoadSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            LoadSource::Global(name) => write!(f, "@{}", name),
            LoadSource::Local(name) => write!(f, "%{}", name),
        }
    }
}

fn load_source(code: &str) -> IResult<&str, LoadSource> {
    alt((
        map(tuple((tag("@"), alphanumeric1)), |(_, name): (_, &str)| {
            LoadSource::Global(name.to_string())
        }),
        map(tuple((tag("%"), alphanumeric1)), |(_, name): (_, &str)| {
            LoadSource::Local(name.to_string())
        }),
    ))(code)
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Load {
    pub from: LoadSource,
    pub to_register: String,
    pub data_type: String,
}

impl Display for Load {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "%{} = load {}* {}",
            self.to_register, self.data_type, self.from
        )
    }
}

pub fn load(code: &str) -> IResult<&str, Load> {
    map(
        tuple((
            tag("%"),
            alphanumeric1,
            space0,
            tag("="),
            space0,
            tag("load"),
            space1,
            alphanumeric1,
            tag("*"),
            space1,
            load_source,
        )),
        |(_, to_register, _, _, _, _, _, data_type, _, _, from)| Load {
            from,
            to_register: to_register.to_string(),
            data_type: data_type.to_string(),
        },
    )(code)
}

impl RegisterCreator for Load {
    fn created(&self) -> &str {
        &self.to_register
    }
}

impl RegisterUser for Load {
    fn used(&self) -> Vec<&str> {
        if let LoadSource::Local(register) = &self.from {
            vec![register]
        } else {
            vec![]
        }
    }
}
