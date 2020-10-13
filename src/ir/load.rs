use crate::ir::register::{parse as parse_register, Register, RegisterRef};
use crate::shared::data_type;
use crate::shared::data_type::Integer;
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
    Local(RegisterRef),
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
        map(parse_register, LoadSource::Local),
    ))(code)
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Load {
    pub from: LoadSource,
    pub to: Register,
}

impl Display for Load {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let to_ref: RegisterRef = (&self.to).into();
        write!(f, "{} = load {}* {}", to_ref, self.to.data_type, self.from)
    }
}

pub fn parse(code: &str) -> IResult<&str, Load> {
    map(
        tuple((
            parse_register,
            space0,
            tag("="),
            space0,
            tag("load"),
            space1,
            data_type::parse_integer,
            tag("*"),
            space1,
            load_source,
        )),
        |(to, _, _, _, _, _, data_type, _, _, from)| Load {
            from,
            to: Register {
                name: to.0,
                data_type: data_type.into(),
            },
        },
    )(code)
}

impl Load {
    pub fn create_register(&self) -> &Register {
        &self.to
    }
    pub fn use_registers(&self) -> Vec<&RegisterRef> {
        if let LoadSource::Local(register) = &self.from {
            vec![register]
        } else {
            Vec::new()
        }
    }
}
