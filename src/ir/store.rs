use crate::ir::register::{parse as parse_register, Register, RegisterRef};
use crate::shared::data_type;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, digit1, space0, space1};
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

sum_type! {
    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum StoreSource {
        NumberLiteral(i64),
        Register(RegisterRef),
    }
}

impl Display for StoreSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            StoreSource::NumberLiteral(name) => write!(f, "{}", name),
            StoreSource::Register(name) => write!(f, "{}", name),
        }
    }
}

fn store_source(code: &str) -> IResult<&str, StoreSource> {
    alt((
        map(parse_register, StoreSource::Register),
        map(digit1, |digits| {
            StoreSource::NumberLiteral(i64::from_str(digits).unwrap())
        }),
    ))(code)
}

sum_type! {
    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum StoreTarget {
        Global(String),
        Local(RegisterRef),
    }
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
        map(parse_register, StoreTarget::Local),
    ))(code)
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Store {
    // todo: pub data_type: String,
    pub source: StoreSource,
    pub target: StoreTarget,
}

impl Display for Store {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "store {}, * {}", self.source, self.target)
    }
}

pub fn parse(code: &str) -> IResult<&str, Store> {
    map(
        tuple((
            tag("store"),
            space1,
            store_source,
            space0,
            tag(","),
            space0,
            data_type::parse_integer,
            tag("*"),
            space1,
            store_target,
        )),
        |(_, _, value, _, _, _, data_type, _, _, target)| Store {
            source: value,
            target,
        },
    )(code)
}

impl Store {
    pub fn use_registers(&self) -> Vec<&RegisterRef> {
        let mut result = if let StoreSource::Register(register) = &self.source {
            vec![register]
        } else {
            vec![]
        };
        if let StoreTarget::Local(register) = &self.target {
            result.push(register);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let code = "store %2, u32* %1";
        let ir = parse(code).unwrap().1;
    }
}
