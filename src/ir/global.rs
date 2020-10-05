use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, space0, space1};
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Global {
    pub name: String,
    // todo: pub data_type: String,
    pub initial_value: i64,
}

impl Display for Global {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "@{} = global {}", self.name, self.initial_value)
    }
}

pub fn parse(code: &str) -> IResult<&str, Global> {
    map(
        tuple((
            tag("@"),
            alphanumeric1,
            space0,
            tag("="),
            space0,
            tag("global"),
            space1,
            alphanumeric1,
        )),
        |(_, name, _, _, _, _, _, initial_value): (_, &str, _, _, _, _, _, _)| Global {
            name: name.to_string(),
            initial_value: i64::from_str(initial_value).unwrap(),
        },
    )(code)
}
