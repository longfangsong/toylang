use crate::parser::statement::Statement;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;

mod context;
mod expression;
mod statement;

pub fn parse(code: &str) -> IResult<&str, Vec<Statement>> {
    many0(map(
        tuple((multispace0, statement::parse, multispace0)),
        |(_, statement, _)| statement,
    ))(code)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::riscv;

    #[test]
    fn it_works() {
        let code = include_str!("./test.toy");
        let statements = parse(code).unwrap().1;
        let irs: Vec<_> = statements.into_iter().map(|it| it.ir()).flatten().collect();
        // println!("{}", riscv::compile(irs));
    }
}
