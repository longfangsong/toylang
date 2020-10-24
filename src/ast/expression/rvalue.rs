use crate::ast::expression::{
    bin_op, bin_op::BinOp, field, field::Field, function_call, function_call::FunctionCall,
    integer_literal, integer_literal::IntegerLiteral, parenthesis, parenthesis::Parenthesis,
    struct_literal, struct_literal::StructLiteral, variable_ref, variable_ref::VariableRef,
};
use nom::{branch::alt, combinator::map, IResult};
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
