use crate::{
    ir::{
        integer_literal,
        integer_literal::IntegerLiteral,
        utils::{global, Global},
    },
    utility::{data_type, data_type::Type},
};
use nom::{
    bytes::complete::tag,
    character::complete::{space0, space1},
    combinator::map,
    sequence::tuple,
    IResult,
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct GlobalDefinition {
    pub item: Global,
    pub data_type: Type,
    pub initial_value: IntegerLiteral,
}

pub fn parse(code: &str) -> IResult<&str, GlobalDefinition> {
    map(
        tuple((
            global::parse,
            space0,
            tag("="),
            space0,
            tag("global"),
            space1,
            data_type::parse,
            space1,
            integer_literal::parse,
        )),
        |(item, _, _, _, _, _, data_type, _, initial_value)| GlobalDefinition {
            item,
            data_type,
            initial_value,
        },
    )(code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse() {
        let code = "@g = global i32 100";
        let result = parse(code).unwrap().1;
        println!("{:?}", result);
    }
}
