use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
use nom::IResult;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, space0, space1};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Global {
    pub name: String,
    pub data_type: String,
    pub initial_value: i64,
}

impl Display for Global {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "@{} = global {} {}", self.name, self.data_type, self.initial_value)
    }
}

pub fn global(code: &str) -> IResult<&str, Global> {
    map(tuple((
        tag("@"), alphanumeric1,
        space0, tag("="), space0,
        tag("global"), space1,
        alphanumeric1, space1,
        alphanumeric1
    )), |(
             _, name,
             _, _, _,
             _, _,
             data_type, _,
             initial_value
         ): (_, &str, _, _, _, _, _, &str, _, &str)| Global {
        name: name.to_string(),
        data_type: data_type.to_string(),
        initial_value: i64::from_str(initial_value).unwrap(),
    })(code)
}