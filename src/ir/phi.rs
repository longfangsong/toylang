use crate::ir::{register, Register, RegisterRef};
use crate::shared::data_type;
use crate::shared::data_type::Type;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, space0};
use nom::combinator::map;
use nom::sequence::{delimited, tuple};
use nom::IResult;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Phi {
    to: Register,
    // todo: do we need a "LabelRef" or "BBRef" type?
    from1: (RegisterRef, String),
    from2: (RegisterRef, String),
}

fn parse_from(code: &str) -> IResult<&str, (RegisterRef, String)> {
    map(
        delimited(
            tag("["),
            tuple((alphanumeric1, space0, tag(","), register::parse)),
            tag("]"),
        ),
        |(label, _, _, reg)| (reg, label.to_string()),
    )(code)
}

pub fn parse(code: &str) -> IResult<&str, Phi> {
    map(
        tuple((
            register::parse,
            space0,
            tag("="),
            space0,
            tag("phi"),
            space0,
            data_type::parse_type,
            space0,
            tag(","),
            parse_from,
            space0,
            tag(","),
            parse_from,
        )),
        |(to, _, _, _, _, _, data_type, _, _, from1, _, _, from2)| Phi {
            to: Register {
                name: to.0,
                data_type,
            },
            from1,
            from2,
        },
    )(code)
}

impl Display for Phi {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} = phi {}, [{}, {}], [{}, {}]",
            self.to.name, self.to.data_type, self.from1.1, self.from1.0, self.from2.1, self.from2.0
        )
    }
}

impl Phi {
    pub fn create_register(&self) -> &Register {
        &self.to
    }
    pub fn use_registers(&self) -> Vec<&RegisterRef> {
        vec![&self.from1.0, &self.from2.0]
    }
}
