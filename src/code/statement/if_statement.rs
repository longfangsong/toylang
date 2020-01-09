use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::combinator::map;
use nom::IResult;
use nom::sequence::tuple;

use crate::code::expression::bin_op::BinOp;
use crate::code::expression::rvalue::RValue;
use crate::code::statement::compound::Compound;
use crate::code::statement::Statement;
use crate::ssa::branch::Branch;
use crate::ssa::label::Label;
use crate::ssa::SSAStatement;
use crate::tools::id_generator::next_id;

pub(crate) struct If<'a> {
    condition: BinOp<'a>,
    when_true: Compound<'a>,
}

impl<'a> Statement<'a> for If<'a> {
    fn generate_ir(&self) -> Vec<Box<dyn SSAStatement + 'a>> {
        let condition_ir = self.condition.generate_ir();
        let when_true_ir = self.when_true.generate_ir();
        let on_true_label_id = next_id();
        let on_true_label_name = format!("L{}", on_true_label_id);
        let on_false_label_id = next_id();
        let on_false_label_name = format!("L{}", on_false_label_id);
        let self_statement = Branch {
            condition: condition_ir.result,
            on_true_label: on_true_label_name.clone(),
            on_false_label: on_false_label_name.clone(),
        };
        let mut result = condition_ir.ssa_generated;
        result.push(Box::new(self_statement));
        result.push(Box::new(Label { name: on_true_label_name }));
        result.extend(when_true_ir);
        result.push(Box::new(Label { name: on_false_label_name }));
        result
    }
}

impl If<'_> {
    pub(crate) fn parse(code: &str) -> IResult<&str, If> {
        map(tuple((tag("if"), space1, BinOp::parse, space1, Compound::parse)),
            |(_, _, condition, _, when_true)| {
                If {
                    condition,
                    when_true,
                }
            })(code)
    }
}

#[test]
fn test_parse() {
    let result = If::parse("if a < 0 {\
    a = 1;\
    }");
    assert!(result.is_ok());
    let result = Compound::parse("if {}");
    assert!(result.is_err());
}

#[test]
fn test_generate_ir() {
    use crate::tools::id_generator::reset_id;
    let result = If::parse("if a < 0 {\
    a = 1;\
    }").unwrap().1;
    reset_id();
    let result = result.generate_ir();
    assert_eq!(result.len(), 9);
}