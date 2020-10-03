use crate::ir::register::{register, Register};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, space0, space1};
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum StoreTarget {
    Global(String),
    Local(Register),
}

impl Display for StoreTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            StoreTarget::Global(name) => write!(f, "@{}", name),
            StoreTarget::Local(name) => write!(f, "{}", name),
        }
    }
}

fn store_target(code: &str) -> IResult<&str, StoreTarget> {
    alt((
        map(tuple((tag("@"), alphanumeric1)), |(_, name): (_, &str)| {
            StoreTarget::Global(name.to_string())
        }),
        map(register, StoreTarget::Local),
    ))(code)
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Store {
    // todo: pub data_type: String,
    pub value: Register,
    pub target: StoreTarget,
}

impl Display for Store {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "store {}, * {}", self.value, self.target)
    }
}

pub fn store(code: &str) -> IResult<&str, Store> {
    map(
        tuple((
            tag("store"),
            space1,
            register,
            space0,
            tag(","),
            space0,
            tag("*"),
            space1,
            store_target,
        )),
        |(_, _, value, _, _, _, _, _, target)| Store { value, target },
    )(code)
}

impl Store {
    pub fn use_registers(&self) -> Vec<&Register> {
        if let StoreTarget::Local(register) = &self.target {
            vec![&self.value, register]
        } else {
            vec![&self.value]
        }
    }
}
