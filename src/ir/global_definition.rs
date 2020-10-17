use crate::shared::data_type::Type;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::sequence::{delimited, tuple};
use nom::IResult;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct GlobalDefinition {
    name: String,
    data_type: Type,
    initial_value: Constant,
}

fn parse(content: &str) -> IResult<&str, GlobalDefinition> {
    map(
        tuple((
            tag("%"),
            parse::ident,
            multispace0,
            data_type::parse,
            constant::parse,
        )),
        |(_, name, _, data_type, initial_value)| GlobalDefinition {
            name,
            data_type,
            initial_value,
        },
    )(code)
}
