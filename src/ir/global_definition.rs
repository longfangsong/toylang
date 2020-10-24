use crate::{
    ir::{
        integer_literal,
        integer_literal::IntegerLiteral,
        utils::{global, Global},
    },
    shared::{data_type, data_type::Type},
};
use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, space0, space1},
    combinator::map,
    sequence::tuple,
    IResult,
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct GlobalDefinition {
    item: Global,
    data_type: Type,
    initial_value: IntegerLiteral,
}

pub fn parse(code: &str) -> IResult<&str, GlobalDefinition> {
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

pub trait GlobalDefinitionVisitor {
    fn visit_global_definition(&mut self, global_definition: &GlobalDefinition);
}
