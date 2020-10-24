use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, space1},
    combinator::map,
    sequence::tuple,
    IResult,
};
use std::{
    fmt,
    fmt::{Display, Formatter},
};

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
    map(
        tuple((tag("j"), space1, alphanumeric1)),
        |(_, _, label): (_, _, &str)| Jump {
            label: label.to_string(),
        },
    )(code)
}
