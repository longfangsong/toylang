use nom::character::complete::{alpha1, alphanumeric0};
use nom::combinator::{map, recognize};
use nom::sequence::pair;
use nom::IResult;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct VariableRef(pub String);

pub fn parse(code: &str) -> IResult<&str, VariableRef> {
    map(recognize(pair(alpha1, alphanumeric0)), |name: &str| {
        VariableRef(name.to_string())
    })(code)
}
