mod assign;
mod declare;

use crate::ir::IR;
use assign::Assign;
use declare::Declare;
use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;

sum_type! {
    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum Statement {
        Declare,
        Assign,
    }
}

pub fn parse(code: &str) -> IResult<&str, Statement> {
    alt((
        map(declare::parse, Statement::Declare),
        map(assign::parse, Statement::Assign),
    ))(code)
}

impl Statement {
    pub fn ir(&self) -> Vec<IR> {
        match self {
            Statement::Declare(declare) => vec![declare.ir()],
            Statement::Assign(assign) => assign.ir(),
        }
    }
}
