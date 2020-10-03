use crate::ir::register::{register, Register};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0, space1};
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

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
    Register(Register),
}

impl Display for Operand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Operand::NumberLiteral(number) => write!(f, "{}", number),
            Operand::Register(register) => write!(f, "{}", register),
        }
    }
}

fn operand(code: &str) -> IResult<&str, Operand> {
    alt((
        map(digit1, |content| {
            Operand::NumberLiteral(i64::from_str(content).unwrap())
        }),
        map(register, Operand::Register),
    ))(code)
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Calculate {
    pub operation: CalculateOperation,
    pub operand1: Operand,
    pub operand2: Operand,
    pub to_register: Register,
}

impl Display for Calculate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} = {} {}, {}",
            self.to_register, self.operation, self.operand1, self.operand2
        )
    }
}

pub fn calculate(code: &str) -> IResult<&str, Calculate> {
    map(
        tuple((
            register,
            space0,
            tag("="),
            space0,
            calculate_operation,
            space1,
            operand,
            space0,
            tag(","),
            space0,
            operand,
        )),
        |(to_register, _, _, _, operation, _, operand1, _, _, _, operand2)| Calculate {
            operation,
            operand1,
            operand2,
            to_register,
        },
    )(code)
}

impl Calculate {
    pub fn create_register(&self) -> &Register {
        &self.to_register
    }

    pub fn use_registers(&self) -> Vec<&Register> {
        let mut result = Vec::new();
        if let Operand::Register(register) = &self.operand1 {
            result.push(register)
        }
        if let Operand::Register(register) = &self.operand2 {
            result.push(register)
        }
        result
    }
}
