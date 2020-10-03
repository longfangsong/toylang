use std::fmt::{self, Display, Formatter};
use nom::IResult;
use nom::combinator::map;
use nom::branch::alt;
use nom::sequence::tuple;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, space1, space0};
use crate::RegisterUser;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum StoreTarget {
    Global(String),
    Local(String),
}

impl Display for StoreTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            StoreTarget::Global(name) => write!(f, "@{}", name),
            StoreTarget::Local(name) => write!(f, "%{}", name)
        }
    }
}

fn store_target(code: &str) -> IResult<&str, StoreTarget> {
    alt((
        map(tuple((tag("@"), alphanumeric1)), |(_, name): (_, &str)| StoreTarget::Global(name.to_string())),
        map(tuple((tag("%"), alphanumeric1)), |(_, name): (_, &str)| StoreTarget::Local(name.to_string())),
    ))(code)
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Store {
    pub data_type: String,
    pub value_register: String,
    pub target: StoreTarget,
}

impl Display for Store {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "store %{}, {}* {}", self.value_register, self.data_type, self.target)
    }
}

pub fn store(code: &str) -> IResult<&str, Store> {
    map(tuple((
        tag("store"), space1,
        tag("%"), alphanumeric1,
        space0, tag(","), space0,
        alphanumeric1, tag("*"), space1,
        store_target
    )), |(
             _, _,
             _, value_register,
             _, _, _,
             data_type, _, _,
             target
         )| Store {
        data_type: data_type.to_string(),
        value_register: value_register.to_string(),
        target,
    })(code)
}

impl RegisterUser for Store {
    fn used(&self) -> Vec<&str> {
        if let StoreTarget::Local(register) = &self.target {
            vec![&self.value_register, register]
        } else {
            vec![&self.value_register]
        }
    }
}