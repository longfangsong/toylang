use crate::shared::parsing;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::pair;
use nom::IResult;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Global(String);

impl Display for Global {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "@{}", self.0)
    }
}

pub fn parse(code: &str) -> IResult<&str, Global> {
    map(pair(tag("@"), parsing::ident), |(_, name)| Global(name))(code)
}
