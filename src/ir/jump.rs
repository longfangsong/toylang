use std::fmt::{self, Display, Formatter};
use nom::IResult;
use nom::sequence::tuple;
use nom::bytes::complete::tag;
use nom::character::complete::{space1, alphanumeric1};
use nom::combinator::map;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Jump {
    pub label: String,
}

impl Display for Jump {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "j {}", self.label)
    }
}

pub fn parse(code: &str) -> IResult<&str, Jump> {
    map(tuple((tag("j"), space1, alphanumeric1)), |(_, _, label): (_, _, &str)| Jump { label: label.to_string() })(code)
}