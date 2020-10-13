mod assign;
mod compound;
mod declare;
mod if_statement;

use crate::ir::IR;
use crate::parser::statement::compound::Compound;
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
        Compound,
    }
}

pub fn parse(code: &str) -> IResult<&str, Statement> {
    alt((
        map(declare::parse, Statement::Declare),
        map(assign::parse, Statement::Assign),
        map(compound::parse, Statement::Compound),
    ))(code)
}

impl Statement {
    pub fn ir(&self) -> Vec<IR> {
        match self {
            Statement::Declare(declare) => vec![declare.ir()],
            Statement::Assign(assign) => assign.ir(),
            Statement::Compound(compound) => compound.ir(),
        }
    }
}
