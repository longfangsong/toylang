use crate::shared::parsing;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, recognize};
use nom::sequence::pair;
use nom::IResult;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Local(String);

impl Display for Local {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "%{}", self.0)
    }
}

pub fn parse(code: &str) -> IResult<&str, Local> {
    map(
        pair(tag("%"), alt((digit1, recognize(parsing::ident)))),
        |(_, name)| Local(name.to_string()),
    )(code)
}
