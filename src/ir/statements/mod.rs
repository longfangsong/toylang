use sum_type::sum_type;

mod alloca;
mod branch;
mod calculate;
mod call;
mod jump;
mod load;
pub mod phi;
mod store;

use alloca::Alloca;
use branch::Branch;
use calculate::Calculate;
use jump::Jump;
use load::Load;
use nom::{branch::alt, combinator::map, IResult};
use store::Store;

sum_type! {
    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum IRStatement {
        Alloca,
        Calculate,
        Load,
        Store,
    }
}

pub fn parse_ir_statement(code: &str) -> IResult<&str, IRStatement> {
    alt((
        map(alloca::parse, IRStatement::Alloca),
        map(calculate::parse, IRStatement::Calculate),
        map(load::parse, IRStatement::Load),
        map(store::parse, IRStatement::Store),
    ))(code)
}

sum_type! {
    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum Terminator {
        Branch,
        Jump,
    }
}

pub fn parse_terminator(code: &str) -> IResult<&str, Terminator> {
    alt((
        map(branch::parse, Terminator::Branch),
        map(jump::parse, Terminator::Jump),
    ))(code)
}
