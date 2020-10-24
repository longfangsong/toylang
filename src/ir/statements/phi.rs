use crate::{
    ir::utils::{local, Local},
    shared::{data_type, data_type::Type, parsing},
};
use nom::{
    bytes::complete::tag,
    character::complete::{space0, space1},
    combinator::map,
    sequence::{delimited, tuple},
    IResult,
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PhiSource {
    name: Local,
    block: String,
}

fn parse_phi_source(code: &str) -> IResult<&str, PhiSource> {
    map(
        delimited(
            tag("["),
            tuple((local::parse, space0, tag(","), space0, parsing::ident)),
            tag("]"),
        ),
        |(name, _, _, _, block)| PhiSource { name, block },
    )(code)
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Phi {
    to: Local,
    data_type: Type,
    from1: PhiSource,
    from2: PhiSource,
}

pub fn parse(code: &str) -> IResult<&str, Phi> {
    map(
        tuple((
            local::parse,
            space0,
            tag("="),
            space0,
            data_type::parse,
            space1,
            parse_phi_source,
            space0,
            tag(","),
            space0,
            parse_phi_source,
        )),
        |(to, _, _, _, data_type, _, from1, _, _, _, from2)| Phi {
            to,
            data_type,
            from1,
            from2,
        },
    )(code)
}
