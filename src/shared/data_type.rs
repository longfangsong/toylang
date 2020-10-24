use crate::shared::parsing;
use fmt::Formatter;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, recognize},
    sequence::pair,
    IResult,
};
use std::{fmt, fmt::Display, str::FromStr};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Integer {
    pub signed: bool,
    pub width: usize,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Type {
    Integer(Integer),
    Struct(String),
    None,
    Address,
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Type::Integer(i) => i.fmt(f),
            Type::Address => write!(f, "address"),
            Type::Struct(name) => write!(f, "{}", name),
            Type::None => write!(f, "()"),
        }
    }
}

impl From<Integer> for Type {
    fn from(integer: Integer) -> Self {
        Type::Integer(integer)
    }
}

pub fn parse_integer(code: &str) -> IResult<&str, Integer> {
    alt((
        map(pair(tag("i"), digit1), |(_, width_str)| Integer {
            signed: true,
            width: usize::from_str(width_str).unwrap(),
        }),
        map(pair(tag("u"), digit1), |(_, width_str)| Integer {
            signed: false,
            width: usize::from_str(width_str).unwrap(),
        }),
    ))(code)
}

pub fn parse(code: &str) -> IResult<&str, Type> {
    alt((
        map(
            alt((recognize(pair(parse_integer, tag("*"))), tag("address"))),
            |_| Type::Address,
        ),
        map(parse_integer, Type::Integer),
        map(parsing::ident, Type::Struct),
        map(tag("()"), |_| Type::None),
    ))(code)
}

impl Display for Integer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", if self.signed { "i" } else { "u" }, self.width)
    }
}
