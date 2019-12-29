use nom::branch::alt;
use nom::IResult;

mod assign;
mod compound;

pub(crate) trait Statement: std::fmt::Debug {
    fn generate_ir(&self) -> String;
}

pub(crate) fn lift<'a, O: 'a + Statement, P>(parser: P) -> impl Fn(&'a str) -> IResult<&'a str, Box<dyn 'a + Statement>>
    where P: Fn(&'a str) -> IResult<&'a str, O> {
    move |code: &'a str| -> IResult<&'a str, Box<dyn 'a + Statement>> {
        parser(code).map(|(rest, result)| (rest, Box::new(result) as _))
    }
}

pub(crate) fn parse<'a>(code: &'a str) -> IResult<&'a str, Box<dyn 'a + Statement>> {
    alt((lift(assign::parse), lift(compound::parse)))(code)
}

