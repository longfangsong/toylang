use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, space1};
use nom::combinator::map;
use nom::IResult;
use nom::multi::many0;
use nom::sequence::{delimited, tuple};

use crate::code::statement;
use crate::code::statement::Statement;
use crate::ssa::SSAStatement;

pub(crate) struct Compound<'a> {
    statements: Vec<Box<dyn 'a + Statement<'a>>>
}

impl<'a> Statement<'a> for Compound<'a> {
    fn generate_ir(&self) -> Vec<Box<dyn SSAStatement + 'a>> {
        self.statements.iter()
            .map(|it| it.generate_ir())
            .flatten()
            .collect()
    }
}


impl Compound<'_> {
    pub(crate) fn parse<'a>(code: &'a str) -> IResult<&'a str, Compound> {
        let parse_with_space = map(tuple((
            many0(alt((line_ending, space1))),
            statement::parse,
            many0(alt((line_ending, space1))))), |(_, r, _)| r);
        map(delimited(tag("{"), many0(parse_with_space), tag("}")),
            |statements: Vec<Box<dyn 'a + Statement>>| {
                Compound { statements }
            })(code)
    }
}

#[test]
fn test_parse() {
    let result = Compound::parse("{\
    a=0;\n
    b=1;\n
    c = a+b;\n
    d = a+b+c;\n
    }");
    assert!(result.is_ok());
    let result = Compound::parse("{}");
    assert!(result.is_ok());
    let result = Compound::parse("{c = a+b;}");
    assert!(result.is_ok());
}

#[test]
fn test_ir() {
    use crate::tools::id_generator::reset_id;
    reset_id();
    let result = Compound::parse("{\
    a=0;\n
    b=1;\n
    c = a+b;\n
    d = a+b+c;\n
    }");
    let generated = result.unwrap().1.generate_ir();
    assert_eq!(generated.len(), 18);
}


