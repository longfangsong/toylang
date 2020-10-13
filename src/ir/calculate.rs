use crate::ir::register::{parse as parse_register, Register, RegisterRef};
use crate::shared::data_type;
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
pub enum Operand {
    NumberLiteral(i64),
    Register(RegisterRef),
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
        map(parse_register, Operand::Register),
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
            "{} = {} {} {}, {}",
            self.to_register.name,
            self.to_register.data_type,
            self.operation,
            self.operand1,
            self.operand2
        )
    }
}

pub fn parse(code: &str) -> IResult<&str, Calculate> {
    map(
        tuple((
            parse_register,
            space0,
            tag("="),
            space0,
            calculate_operation,
            space1,
            data_type::parse_integer,
            space1,
            operand,
            space0,
            tag(","),
            space0,
            operand,
        )),
        |(to_register, _, _, _, operation, _, data_type, _, operand1, _, _, _, operand2)| {
            Calculate {
                operation,
                operand1,
                operand2,
                to_register: Register {
                    name: to_register.0,
                    data_type: data_type.into(),
                },
            }
        },
    )(code)
}

impl Calculate {
    pub fn create_register(&self) -> &Register {
        &self.to_register
    }

    pub fn use_registers(&self) -> Vec<&RegisterRef> {
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
