use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Register(pub String);

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "%{}", self.0)
    }
}

pub fn parse(code: &str) -> IResult<&str, Register> {
    map(tuple((tag("%"), alphanumeric1)), |(_, name): (_, &str)| {
        Register(name.to_string())
    })(code)
}
