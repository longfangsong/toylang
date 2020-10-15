use crate::ast::expression::bin_op::BinOp;
use crate::ast::expression::field::Field;
use crate::ast::expression::function_call::FunctionCall;
use crate::ast::expression::integer_literal::IntegerLiteral;
use crate::ast::expression::parenthesis::Parenthesis;
use crate::ast::expression::struct_literal::StructLiteral;
use crate::ast::expression::variable_ref::VariableRef;
use crate::ast::expression::{
    bin_op, field, function_call, integer_literal, parenthesis, struct_literal, variable_ref,
};
use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;
use sum_type::sum_type;

sum_type! {
    #[derive(Debug, Eq, PartialEq, Clone, Hash)]
    pub enum RValue {
        IntegerLiteral,
        VariableRef,
        Parenthesis,
        BinOp,
        Field,
        FunctionCall,
        StructLiteral,
    }
}

pub fn parse(code: &str) -> IResult<&str, RValue> {
    alt((
        map(struct_literal::parse, RValue::StructLiteral),
        map(function_call::parse, RValue::FunctionCall),
        map(bin_op::parse, RValue::BinOp),
        map(field::parse, RValue::Field),
        map(integer_literal::parse, RValue::IntegerLiteral),
        map(variable_ref::parse, RValue::VariableRef),
        map(parenthesis::parse, RValue::Parenthesis),
    ))(code)
}
