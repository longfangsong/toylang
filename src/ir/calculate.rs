use std::fmt::{self, Display, Formatter};
use nom::IResult;
use nom::branch::alt;
use nom::combinator::map;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, alphanumeric1, space0, space1};
use std::str::FromStr;
use nom::sequence::tuple;
use crate::{RegisterCreator, RegisterUser};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CalculateOperation {
    Add,
    Less,
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
        map(tag("less"), |_| CalculateOperation::Less),
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
pub enum Operand {
    NumberLiteral(i64),
    Register(String),
}

impl Display for Operand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Operand::NumberLiteral(number) => write!(f, "{}", number),
            Operand::Register(name) => write!(f, "%{}", name),
        }
    }
}

fn operand(code: &str) -> IResult<&str, Operand> {
    alt((
        map(digit1, |content| Operand::NumberLiteral(i64::from_str(content).unwrap())),
        map(tuple((tag("%"), alphanumeric1)), |(_, content): (_, &str)| Operand::Register(content.to_string())),
    ))(code)
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Calculate {
    pub operation: CalculateOperation,
    pub operand1: Operand,
    pub operand2: Operand,
    pub to_register: String,
}

impl Display for Calculate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "%{} = {} {}, {}", self.to_register, self.operation, self.operand1, self.operand2)
    }
}

pub fn calculate(code: &str) -> IResult<&str, Calculate> {
    map(tuple((
        tag("%"), alphanumeric1,
        space0, tag("="), space0,
        calculate_operation, space1,
        operand,
        space0, tag(","), space0,
        operand
    )), |(
             _, to_register,
             _, _, _,
             calculate_operation, _,
             operand1,
             _, _, _,
             operand2
         )| Calculate {
        operation: calculate_operation,
        operand1,
        operand2,
        to_register: to_register.to_string(),
    })(code)
}

impl RegisterCreator for Calculate {
    fn created(&self) -> &str {
        &self.to_register
    }
}

impl RegisterUser for Calculate {
    fn used(&self) -> Vec<&str> {
        let mut result: Vec<&str> = Vec::new();
        if let Operand::Register(register) = &self.operand1 {
            result.push(register)
        }
        if let Operand::Register(register) = &self.operand2 {
            result.push(register)
        }
        result
    }
}