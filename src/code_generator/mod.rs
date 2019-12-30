use std::collections::BTreeSet;

use nom::branch::alt;
use nom::character::complete::{line_ending, space1};
use nom::combinator::map;
use nom::IResult;
use nom::lib::std::collections::BTreeMap;
use nom::multi::many0;
use nom::sequence::tuple;

use crate::code_generator::register::assign::assign;

mod load_address;
mod load_instant;
mod load_variable;
mod add;
mod store;
pub mod register;

pub trait CodeGenerator<'a>: std::fmt::Debug {
    fn generate_asm(&self) -> String;

    // think I'll use a 32-bit bitfield?
    // no, I'm not that kind of hack
    fn using_regs(&self) -> BTreeSet<&register::Register>;

    fn assign_regs(&self, dict: &BTreeMap<&register::Register, register::Register>) -> Box<dyn CodeGenerator<'a> + 'a>;
}

pub(crate) fn lift<'a, O: 'a + CodeGenerator<'a>, P>(parser: P) -> impl Fn(&'a str) -> IResult<&'a str, Box<dyn CodeGenerator<'a> + 'a>>
    where P: Fn(&'a str) -> IResult<&'a str, O> {
    move |code: &'a str| -> IResult<&'a str, Box<dyn CodeGenerator<'a> + 'a>> {
        parser(code).map(|(rest, result)| (rest, Box::new(result) as _))
    }
}

fn parse_ir_line<'a>(ir: &'a str) -> IResult<&'a str, Box<dyn 'a + CodeGenerator>> {
    alt((
        lift(load_instant::parse),
        lift(load_address::parse),
        lift(load_variable::parse),
        lift(store::parse),
        lift(add::parse),
    ))(ir)
}

pub(crate) fn parse_ir<'a>(ir: &'a str) -> IResult<&'a str, Vec<Box<dyn 'a + CodeGenerator>>> {
    let parse_with_space = map(tuple((
        many0(alt((line_ending, space1))),
        parse_ir_line,
        many0(alt((line_ending, space1))))), |(_, r, _)| r);
    many0(parse_with_space)(ir)
}

pub fn assign_registers<'a>(generators: &[Box<dyn CodeGenerator<'a> + 'a>]) -> Vec<Box<dyn CodeGenerator<'a> + 'a>> {
    let registers: Vec<_> = generators.iter()
        .map(|it| it.using_regs())
        .collect();
    let dict = assign(registers);
    generators.iter().map(|it| it.assign_regs(&dict)).collect()
}

#[test]
fn test_parse_ir() {
    let result = parse_ir("\
    %0 = 1;
%1 = &a;
*%1 = %0;
%2 = 2;
%3 = &b;
*%3 = %2;
%4 = a;
%5 = b;
%6 = add %4, %5;
%7 = &c;
*%7 = %6;
    ");
    assert_eq!(11, result.unwrap().1.len());
}
