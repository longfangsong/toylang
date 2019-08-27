use crate::symbol_table::table::SymbolTable;

pub mod expression;
pub mod statement;

#[derive(Clone)]
pub struct Context<'a> {
    symbol_table: &'a SymbolTable
}