use crate::shared::data_type::{Integer, Type};
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::sync::RwLock;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Register {
    pub name: String,
    pub data_type: Type,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct RegisterRef(pub String);

impl From<&Register> for RegisterRef {
    fn from(register: &Register) -> Self {
        RegisterRef(register.name.clone())
    }
}

impl Display for RegisterRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "%{}", self.0)
    }
}

pub fn parse(code: &str) -> IResult<&str, RegisterRef> {
    map(tuple((tag("%"), alphanumeric1)), |(_, name): (_, &str)| {
        RegisterRef(name.to_string())
    })(code)
}

lazy_static! {
    static ref REGISTER_SYMBOL_TABLE: RwLock<HashMap<RegisterRef, Register>> =
        RwLock::new(HashMap::new());
}
