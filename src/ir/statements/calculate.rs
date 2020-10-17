use crate::ir::utils::LocalOrNumberLiteral;
use crate::shared::data_type;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0, space1};
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CalculateOperation {
    Add,
    LT,
    LE,
    GT,
    GE,
    EQ,
    NE,
    Sub,
    Or,
    Xor,
    And,
    SLL,
    SRL,
    SRA,
}

impl Display for CalculateOperation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_ascii_lowercase())
    }
}

fn calculate_operation(code: &str) -> IResult<&str, CalculateOperation> {
    alt((
        map(tag("add"), |_| CalculateOperation::Add),
        map(tag("less"), |_| CalculateOperation::LT),
        map(tag("sub"), |_| CalculateOperation::Sub),
        map(tag("or"), |_| CalculateOperation::Or),
        map(tag("xor"), |_| CalculateOperation::Xor),
        map(tag("and"), |_| CalculateOperation::And),
        map(tag("sll"), |_| CalculateOperation::SLL),
        map(tag("srl"), |_| CalculateOperation::SRL),
        map(tag("sra"), |_| CalculateOperation::SRA),
    ))(code)
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Calculate {
    pub operation: CalculateOperation,
    pub operand1: LocalOrNumberLiteral,
    pub operand2: LocalOrNumberLiteral,
    pub to_register: String,
}
