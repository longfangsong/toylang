use crate::ast::expression::bin_op::BinOp;
use crate::ast::expression::constant::Constant;
use crate::ast::expression::field::Field;
use crate::ast::expression::function_call::FunctionCall;
use crate::ast::expression::parenthesis::Parenthesis;
use crate::ast::expression::variable_ref::VariableRef;
use crate::ast::expression::{bin_op, constant, field, function_call, parenthesis, variable_ref};
use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;
use sum_type::sum_type;

sum_type! {
    #[derive(Debug, Eq, PartialEq, Clone, Hash)]
    pub enum RValue {
        Constant,
        VariableRef,
        Parenthesis,
        BinOp,
        Field,
        FunctionCall,
    }
}

pub fn parse(code: &str) -> IResult<&str, RValue> {
    alt((
        map(bin_op::parse, RValue::BinOp),
        map(function_call::parse, RValue::FunctionCall),
        map(field::parse, RValue::Field),
        map(constant::parse, RValue::Constant),
        map(variable_ref::parse, RValue::VariableRef),
        map(parenthesis::parse, RValue::Parenthesis),
    ))(code)
}
