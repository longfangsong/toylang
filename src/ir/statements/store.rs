use crate::{
    ir::utils::{local_or_global, local_or_number_literal, LocalOrGlobal, LocalOrNumberLiteral},
    utility::{data_type, data_type::Type},
};
use nom::{
    bytes::complete::tag,
    character::{complete::space1, streaming::space0},
    combinator::map,
    sequence::tuple,
    IResult,
};

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
