use crate::ir::register::{register, Register};
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
    Local(Register),
}

impl Display for LoadSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            LoadSource::Global(name) => write!(f, "@{}", name),
            LoadSource::Local(register) => write!(f, "{}", register),
        }
    }
}

fn load_source(code: &str) -> IResult<&str, LoadSource> {
    alt((
        map(tuple((tag("@"), alphanumeric1)), |(_, name): (_, &str)| {
            LoadSource::Global(name.to_string())
        }),
        map(register, LoadSource::Local),
    ))(code)
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Load {
    pub from: LoadSource,
    pub to: Register,
    // todo: pub data_type: String,
}

impl Display for Load {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} = load * {}", self.to, self.from)
    }
}

pub fn load(code: &str) -> IResult<&str, Load> {
    map(
        tuple((
            register,
            space0,
            tag("="),
            space0,
            tag("load"),
            space1,
            tag("*"),
            space1,
            load_source,
        )),
        |(to, _, _, _, _, _, _, _, from)| Load { from, to },
    )(code)
}

impl Load {
    pub fn create_register(&self) -> &Register {
        &self.to
    }
    pub fn use_registers(&self) -> Vec<&Register> {
        if let LoadSource::Local(register) = &self.from {
            vec![register]
        } else {
            Vec::new()
        }
    }
}