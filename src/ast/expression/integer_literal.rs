use crate::shared::parsing;
use nom::combinator::map;
use nom::IResult;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct IntegerLiteral(pub i64);

impl From<i64> for IntegerLiteral {
    fn from(i: i64) -> Self {
        IntegerLiteral(i)
    }
}

pub fn parse(code: &str) -> IResult<&str, IntegerLiteral> {
    map(parsing::integer, IntegerLiteral)(code)
}
