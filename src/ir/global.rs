use crate::shared::data_type;
use crate::shared::data_type::Integer;
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
    pub data_type: Integer,
    pub initial_value: i64,
}

impl Display for Global {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "@{} = global {} {}",
            self.name, self.data_type, self.initial_value
        )
    }
}

pub fn parse(code: &str) -> IResult<&str, Global> {
    map(
        tuple((
            tag("@"),
            map(alphanumeric1, |it: &str| it.to_string()),
            space0,
            tag("="),
            space0,
            tag("global"),
            space1,
            data_type::parse,
            space1,
            alphanumeric1,
        )),
        |(_, name, _, _, _, _, _, data_type, _, initial_value)| Global {
            name,
            data_type,
            initial_value: i64::from_str(initial_value).unwrap(),
        },
    )(code)
}
