use std::fmt::{self, Display, Formatter};
use nom::IResult;
use nom::branch::alt;
use nom::combinator::map;
use nom::bytes::complete::tag;
use nom::sequence::tuple;
use nom::character::complete::{alphanumeric1, space1, space0};
use crate::RegisterUser;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Branch {
    pub branch_type: BranchType,
    pub operand1: String,
    pub operand2: String,
    pub success_label: String,
    pub failure_label: String,
}

impl Display for Branch {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "b{} %{}, %{}, {}, {}", self.branch_type, self.operand1, self.operand2, self.success_label, self.failure_label)
    }
}

pub fn branch(code: &str) -> IResult<&str, Branch> {
    map(tuple((
        tag("b"), branch_type, space1,
        tag("%"), alphanumeric1,
        space0, tag(","), space0,
        tag("%"), alphanumeric1,
        space0, tag(","), space0,
        alphanumeric1,
        space0, tag(","), space0,
        alphanumeric1
    )), |(
             _, branch_type, _,
             _, operand1,
             _, _, _,
             _, operand2,
             _, _, _,
             success_label,
             _, _, _,
             failure_label
         )| Branch {
        branch_type,
        operand1: operand1.to_string(),
        operand2: operand2.to_string(),
        success_label: success_label.to_string(),
        failure_label: failure_label.to_string(),
    })(code)
}

impl RegisterUser for Branch {
    fn used(&self) -> Vec<&str> {
        vec![&self.operand1, &self.operand2]
    }
}
