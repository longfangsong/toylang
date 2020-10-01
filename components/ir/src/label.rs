use std::fmt::{self, Display, Formatter};
use nom::IResult;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::character::complete::alphanumeric1;
use nom::bytes::complete::tag;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Label(pub String);

impl Display for Label {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}:", self.0)
    }
}

pub fn label(code: &str) -> IResult<&str, Label> {
    map(tuple((alphanumeric1, tag(":"))), |(label, _): (&str, _)| Label(label.to_string()))(code)
}