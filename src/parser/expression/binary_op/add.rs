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

pub struct Add {
    pub left: Box<dyn RValue>,
    pub right: Box<dyn RValue>,
}

impl RValue for Add {
    fn generate_rvalue(&self) -> ExpressionParseResult {
        let left_result = self.left.generate_rvalue();
        let right_result = self.right.generate_rvalue();
        if left_result.type_name != right_result.type_name {
            panic!("Cannot add two variables of different types!")
        }
        let typename = left_result.type_name;
        let bind_to = format!("%add_result_temp_{}", SEQUENCE.next());
        let code = format!("{} = add nsw {} {}, {}", bind_to, typename, left_result.bind_to, right_result.bind_to);
        let left_code = left_result.generated_code.to_owned() + if left_result.generated_code.len() != 0 {
            "\n"
        } else {
            ""
        };
        let right_code = right_result.generated_code.to_owned() + if right_result.generated_code.len() != 0 {
            "\n"
        } else {
            ""
        };
        ExpressionParseResult {
            type_name: typename,
            generated_code: left_code + right_code.as_str() + code.as_str(),
            bind_to,
        }
    }
}

pub fn parser_add<'a>(input: (&'a str, Context<'a>)) -> IResult<(&'a str, Context<'a>), Add> {
    map(lift(tuple((
        priority_less_than_1,
        space0,
        tag("+"),
        space0,
        rvalue
    ))), |(left, _, _, _, right)|
            Add { left: Box::new(left), right: Box::new(right) },
    )(input)
}