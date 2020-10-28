use crate::ir::utils::Local;
use nom::{branch::alt, combinator::map, IResult};
use sum_type::sum_type;
use union_type::union_type;

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

union_type! {
    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum IRStatement {
        Alloca,
        Calculate,
        Load,
        Store,
        LoadField,
    }

    impl IRStatement {
        pub fn used_registers(&self) -> Vec<&Local>;

        pub fn create_register(&self) -> Option<&Local>;
    }
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

sum_type! {
    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum Terminator {
        Branch,
        Jump,
        Ret,
    }
}

pub fn parse_terminator(code: &str) -> IResult<&str, Terminator> {
    alt((
        map(branch::parse, Terminator::Branch),
        map(jump::parse, Terminator::Jump),
        map(ret::parse, Terminator::Ret),
    ))(code)
}

pub trait IRStatementVisitor {
    fn visit_ir_statement(&mut self);
}

pub trait TerminatorVisitor {
    fn visit_terminator(&mut self);
}
