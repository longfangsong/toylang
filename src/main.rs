#[macro_use]
extern crate lazy_static;

use crate::parser::expression::binary_op::add::Add;
use crate::parser::expression::number_literal::{float_literal, integer_literal};
use crate::parser::expression::rvalue::RValue;
use crate::parser::expression::variable_reference::VariableReference;
use crate::symbol_table::symbol::Symbol;
use crate::symbol_table::table::SymbolTable;
use crate::util::sequence::SEQUENCE;

mod parser;
mod symbol_table;
mod util;

fn main() {
    let mut symbol_table = SymbolTable::new();
    symbol_table.add_entry("i32", "a");
    symbol_table.push();
    symbol_table.add_entry("i32", "b");
    let a_ref = VariableReference { variable: symbol_table.find_entry("a").unwrap() };
    let b_ref = VariableReference { variable: symbol_table.find_entry("b").unwrap() };
    let add = Add {
        left: Box::new(a_ref),
        right: Box::new(1_i32),
    };
    println!("{:?}", add.generate_rvalue());
}
