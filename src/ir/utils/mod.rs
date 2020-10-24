pub mod global;
pub mod local;

pub use crate::ir::utils::{global::Global, local::Local};
use crate::shared::parsing;
use nom::{branch::alt, combinator::map, IResult};
use std::fmt::{self, Display, Formatter};
use sum_type::sum_type;

sum_type! {
    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum LocalOrNumberLiteral {
        Local(Local),
        NumberLiteral(i64),
    }
}

pub fn local_or_number_literal(code: &str) -> IResult<&str, LocalOrNumberLiteral> {
    alt((
        map(local::parse, LocalOrNumberLiteral::Local),
        map(parsing::integer, LocalOrNumberLiteral::NumberLiteral),
    ))(code)
}

impl Display for LocalOrNumberLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            LocalOrNumberLiteral::Local(local) => write!(f, "%{}", local),
            LocalOrNumberLiteral::NumberLiteral(number) => write!(f, "{}", number),
        }
    }
}

sum_type! {
    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum LocalOrGlobal {
       Local(Local),
       Global(Global),
    }
}

pub fn local_or_global(code: &str) -> IResult<&str, LocalOrGlobal> {
    alt((
        map(local::parse, LocalOrGlobal::Local),
        map(global::parse, LocalOrGlobal::Global),
    ))(code)
}

impl Display for LocalOrGlobal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            LocalOrGlobal::Local(local) => write!(f, "{}", local),
            LocalOrGlobal::Global(global) => write!(f, "{}", global),
        }
    }
}
