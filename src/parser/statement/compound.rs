use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, space1};
use nom::combinator::map;
use nom::IResult;
use nom::multi::many0;
use nom::sequence::{delimited, tuple};

use crate::parser::statement;
use crate::parser::statement::Statement;

#[derive(Debug)]
pub(crate) struct Compound<'a> {
    statements: Vec<Box<dyn 'a + Statement>>
}

impl Statement for Compound<'_> {
    fn generate_ir(&self) -> String {
        self.statements.iter()
            .map(|it| it.generate_ir())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

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

#[test]
fn test_parse() {
    let result = parse("{\
    a=0;\n
    b=1;\n
    c = a+b;\n
    d = a+b+c;\n
    }");
    assert!(result.is_ok());
    let result = parse("{}");
    assert!(result.is_ok());
    let result = parse("{c = a+b;}");
    assert!(result.is_ok());
}

#[test]
fn test_ir() {
    use crate::tools::id_generator::reset_id;
    reset_id();
    let result = parse("{\
    a=0;\n
    b=1;\n
    c = a+b;\n
    d = a+b+c;\n
    }");
    let generated = result.unwrap().1.generate_ir();
    assert_eq!(generated, "%0 = 0;
%1 = &a;
*%1 = %0;
%2 = 1;
%3 = &b;
*%3 = %2;
%4 = a;
%5 = b;
%6 = add %4, %5;
%7 = &c;
*%7 = %6;
%8 = a;
%9 = b;
%10 = c;
%11 = add %9, %10;
%12 = add %8, %11;
%13 = &d;
*%13 = %12;");
}