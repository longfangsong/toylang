use crate::ir::load::LoadSource;
use crate::ir::{Load, Store};
use crate::parser::context::CONTEXT;
use crate::parser::expression::ExpressionResult;
use nom::character::complete::{alpha1, alphanumeric0};
use nom::combinator::{map, recognize};
use nom::sequence::pair;
use nom::IResult;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Variable(pub String);

pub fn parse(code: &str) -> IResult<&str, Variable> {
    map(recognize(pair(alpha1, alphanumeric0)), |name: &str| {
        Variable(name.to_string())
    })(code)
}

impl Variable {
    pub fn ir(&self) -> ExpressionResult {
        let result = CONTEXT.next();
        ExpressionResult::Complex {
            ir_generated: vec![Load {
                from: LoadSource::Global(self.0.clone()),
                to: result.clone(),
            }
            .into()],
            result,
        }
    }
}