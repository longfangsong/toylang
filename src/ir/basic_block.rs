use crate::ir;
use crate::ir::{label, phi, Label, Phi, IR};
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::{delimited, tuple};
use nom::IResult;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BasicBlock {
    name: Label,
    phis: Vec<Phi>,
    content: Vec<IR>,
}

pub fn parse(code: &str) -> IResult<&str, BasicBlock> {
    map(
        tuple((
            label::parse,
            many0(delimited(multispace0, phi::parse, multispace0)),
            many0(delimited(multispace0, ir::parse, multispace0)),
        )),
        |(name, phis, content)| BasicBlock {
            name,
            phis,
            content,
        },
    )(code)
}

impl Display for BasicBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.name)?;
        for phi in self.phis {
            writeln!(f, "    {}", phi)?;
        }
        for content in self.content {
            writeln!(f, "    {}", content)?;
        }
        Ok(())
    }
}
