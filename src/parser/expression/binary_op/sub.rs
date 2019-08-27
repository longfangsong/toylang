use nom::bytes::complete::tag;
use nom::character::complete::{char, space0};
use nom::combinator::map;
use nom::IResult;
use nom::sequence::tuple;

use crate::parser::Context;
use crate::parser::expression::binary_op::priority_less_than_1;
use crate::parser::expression::result::ExpressionParseResult;
use crate::parser::expression::rvalue::{RValue, rvalue};
use crate::symbol_table::table::SymbolTable;
use crate::util::lift::lift;
use crate::util::sequence::SEQUENCE;

pub struct Sub {
    pub left: Box<dyn RValue>,
    pub right: Box<dyn RValue>,
}

impl RValue for Sub {
    fn generate_rvalue(&self) -> ExpressionParseResult {
        let left_result = self.left.generate_rvalue();
        let right_result = self.right.generate_rvalue();
        if left_result.type_name != right_result.type_name {
            panic!("Cannot sub two variables of different types!")
        }
        let typename = left_result.type_name;
        let bind_to = format!("%sub_result_temp_{}", SEQUENCE.next());
        let code = format!("{} = sub nsw {} {},{}", bind_to, typename, left_result.bind_to, right_result.bind_to);
        ExpressionParseResult {
            type_name: typename,
            generated_code: left_result.generated_code + "\n" + right_result.generated_code.as_str() + "\n" + code.as_str(),
            bind_to,
        }
    }
}

pub fn parser_sub<'a>(input: (&'a str, Context<'a>)) -> IResult<(&'a str, Context<'a>), Sub> {
    map(lift(tuple((
        priority_less_than_1,
        space0,
        tag("-"),
        space0,
        rvalue
    ))), |(left, _, _, _, right)|
            Sub { left: Box::new(left), right: Box::new(right) },
    )(input)
}