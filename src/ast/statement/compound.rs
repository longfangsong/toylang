use crate::ast::context::CONTEXT;
use crate::ast::statement;
use crate::ast::statement::{compound, Statement};
use crate::ir::IR;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::{delimited, tuple};
use nom::IResult;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Compound(Vec<Statement>);

pub fn parse(code: &str) -> IResult<&str, Compound> {
    map(
        delimited(
            tag("{"),
            many0(map(
                tuple((multispace0, statement::parse, multispace0)),
                |(_, it, _)| it,
            )),
            tag("}"),
        ),
        Compound,
    )(code)
}

impl Compound {
    pub fn ir(&self) -> Vec<IR> {
        self.0.iter().flat_map(|it| it.ir()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::data_type::Integer;

    #[test]
    fn it_works() {
        CONTEXT.insert_variable(
            "c",
            Integer {
                signed: true,
                width: 32,
            },
        );
        CONTEXT.insert_variable(
            "d",
            Integer {
                signed: true,
                width: 32,
            },
        );
        let item = parse("{ c = d; }").unwrap().1;
        let ir = item.ir();
        // todo: more tests
        assert_eq!(ir.len(), 2);
    }
}
