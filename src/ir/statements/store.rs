use crate::ir::utils::{
    local_or_global, local_or_number_literal, LocalOrGlobal, LocalOrNumberLiteral,
};
use crate::shared::data_type;
use crate::shared::data_type::Type;
use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::character::streaming::space0;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Store {
    pub data_type: Type,
    pub source: LocalOrNumberLiteral,
    pub target: LocalOrGlobal,
}

pub fn parse(code: &str) -> IResult<&str, Store> {
    map(
        tuple((
            tag("store"),
            space1,
            data_type::parse,
            space1,
            local_or_number_literal,
            space0,
            tag(","),
            space0,
            tag("address"),
            space1,
            local_or_global,
        )),
        |(_, _, data_type, _, source, _, _, _, _, _, target)| Store {
            data_type,
            source,
            target,
        },
    )(code)
}
