use crate::{
    ir::utils::{local, local_or_number_literal, Local, LocalOrNumberLiteral},
    shared::{data_type, data_type::Type},
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{space0, space1},
    combinator::map,
    sequence::tuple,
    IResult,
};
use std::{
    fmt,
    fmt::{Display, Formatter},
};

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
    pub to: Local,
    pub data_type: Type,
}

impl Calculate {
    pub fn used_registers(&self) -> Vec<&Local> {
        let mut result = Vec::with_capacity(2);
        if let LocalOrNumberLiteral::Local(op1) = &self.operand1 {
            result.push(op1)
        }
        if let LocalOrNumberLiteral::Local(op2) = &self.operand2 {
            result.push(op2)
        }
        result
    }

    fn create_register(&self) -> Option<&Local> {
        Some(&self.to)
    }
}

pub fn parse(code: &str) -> IResult<&str, Calculate> {
    map(
        tuple((
            local::parse,
            space0,
            tag("="),
            space0,
            calculate_operation,
            space1,
            data_type::parse,
            space1,
            local_or_number_literal,
            space0,
            tag(","),
            space0,
            local_or_number_literal,
        )),
        |(to_register, _, _, _, operation, _, data_type, _, operand1, _, _, _, operand2)| {
            Calculate {
                operation,
                operand1,
                operand2,
                to: to_register,
                data_type,
            }
        },
    )(code)
}
