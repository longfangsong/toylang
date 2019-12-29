use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::IResult;
use nom::sequence::pair;

pub(crate) fn parse(ir: &str) -> IResult<&str, u64> {
    map(pair(tag("%"), digit1), |(_, digits)| {
        u64::from_str(digits).unwrap()
    })(ir)
}