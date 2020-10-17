use crate::ir::utils::LocalOrNumberLiteral;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::IResult;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum BranchType {
    EQ,
    NE,
    LT,
    GE,
}

fn branch_type(code: &str) -> IResult<&str, BranchType> {
    alt((
        map(tag("eq"), |_| BranchType::EQ),
        map(tag("ne"), |_| BranchType::NE),
        map(tag("lt"), |_| BranchType::LT),
        map(tag("ge"), |_| BranchType::GE),
    ))(code)
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Branch {
    pub branch_type: BranchType,
    pub operand1: LocalOrNumberLiteral,
    pub operand2: LocalOrNumberLiteral,
    pub success_label: String,
    pub failure_label: String,
}
