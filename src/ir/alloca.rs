use crate::ir::register::parse as parse_register;
use crate::ir::Register;
use crate::shared::data_type;
use crate::shared::data_type::Type::Address;
use crate::shared::data_type::{Integer, Type};
use nom::bytes::complete::tag;
use nom::character::complete::{space0, space1};
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Alloca {
    pub to: Register,
    pub alloc_type: Type,
}

impl Display for Alloca {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} = alloca {}", self.to.name, self.to.data_type)
    }
}

pub fn parse(code: &str) -> IResult<&str, Alloca> {
    map(
        tuple((
            parse_register,
            space0,
            tag("="),
            space0,
            tag("alloca"),
            space1,
            data_type::parse_integer,
        )),
        |(to, _, _, _, _, _, data_type)| Alloca {
            to: Register {
                name: to.0,
                data_type: Address,
            },
            alloc_type: Type::Integer(data_type),
        },
    )(code)
}

impl Alloca {
    pub fn alloc_space(&self) -> usize {
        4
    }
}
