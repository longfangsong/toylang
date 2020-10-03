use crate::ir::register::{register, Register};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, space0, space1};
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;
use std::fmt::{self, Display, Formatter};

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

impl Display for BranchType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_ascii_lowercase())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Branch {
    pub branch_type: BranchType,
    pub operand1: Register,
    pub operand2: Register,
    pub success_label: String,
    pub failure_label: String,
}

impl Display for Branch {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "b{} {}, {}, {}, {}",
            self.branch_type, self.operand1, self.operand2, self.success_label, self.failure_label
        )
    }
}

pub fn branch(code: &str) -> IResult<&str, Branch> {
    map(
        tuple((
            tag("b"),
            branch_type,
            space1,
            register,
            space0,
            tag(","),
            space0,
            register,
            space0,
            tag(","),
            space0,
            alphanumeric1,
            space0,
            tag(","),
            space0,
            alphanumeric1,
        )),
        |(
             _,
             branch_type,
             _,
             operand1,
             _,
             _,
             _,
             operand2,
             _,
             _,
             _,
             success_label,
             _,
             _,
             _,
             failure_label,
         )| Branch {
            branch_type,
            operand1,
            operand2,
            success_label: success_label.to_string(),
            failure_label: failure_label.to_string(),
        },
    )(code)
}

impl Branch {
    pub fn use_registers(&self) -> Vec<&Register> {
        vec![&self.operand1, &self.operand2]
    }
}
