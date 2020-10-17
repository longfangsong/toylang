use crate::ir::integer_literal;
use crate::ir::integer_literal::IntegerLiteral;
use crate::ir::utils::{global, Global};
use crate::shared::data_type;
use crate::shared::data_type::Type;
use nom::bytes::complete::tag;
use nom::character::complete::{multispace0, space0, space1};
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct GlobalDefinition {
    item: Global,
    data_type: Type,
    initial_value: IntegerLiteral,
}

fn parse(code: &str) -> IResult<&str, GlobalDefinition> {
    map(
        tuple((
            global::parse,
            space0,
            tag("="),
            space0,
            tag("global"),
            data_type::parse,
            space1,
            integer_literal::parse,
        )),
        |(item, _, _, _, _, data_type, _, initial_value)| GlobalDefinition {
            item,
            data_type,
            initial_value,
        },
    )(code)
}
