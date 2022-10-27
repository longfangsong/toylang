use enum_dispatch::enum_dispatch;
use nom::{branch::alt, combinator::map, IResult};

mod alloca;
mod branch;
mod calculate;
mod call;
mod jump;
mod load;
mod load_field;
pub mod phi;
mod ret;
mod store;

use alloca::Alloca;
use branch::Branch;
use calculate::Calculate;
use jump::Jump;
use load::Load;
use load_field::LoadField;
use ret::Ret;
use store::Store;

#[enum_dispatch]
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum IRStatement {
    Alloca,
    Calculate,
    Load,
    Store,
    LoadField,
}
pub fn parse_ir_statement(code: &str) -> IResult<&str, IRStatement> {
    alt((
        map(alloca::parse, IRStatement::Alloca),
        map(calculate::parse, IRStatement::Calculate),
        map(load_field::parse, IRStatement::LoadField),
        map(load::parse, IRStatement::Load),
        map(store::parse, IRStatement::Store),
    ))(code)
}

#[enum_dispatch]
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Terminator {
    Branch,
    Jump,
    Ret,
}

pub fn parse_terminator(code: &str) -> IResult<&str, Terminator> {
    alt((
        map(branch::parse, Terminator::Branch),
        map(jump::parse, Terminator::Jump),
        map(ret::parse, Terminator::Ret),
    ))(code)
}
