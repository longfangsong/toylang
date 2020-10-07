mod symbol_table;

use crate::ir::Register as LogicalRegister;
use crate::parser::context::symbol_table::{Entry, SymbolTable};
use crate::parser::expression::variable_ref::VariableRef;
use crate::shared::data_type::Integer;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::RwLock;

pub struct Context {
    next_logical_register: AtomicUsize,
    symbol_table: RwLock<SymbolTable>,
}

impl Context {
    pub fn reset(&self) {
        self.next_logical_register.store(0, Ordering::Relaxed);
    }

    pub fn next(&self) -> LogicalRegister {
        let result = self.next_logical_register.fetch_add(1, Ordering::Relaxed);
        LogicalRegister(format!("{}", result))
    }

    pub fn insert_variable(&self, variable_name: &str, variable_type: Integer) {
        let next_logical_register = self.next();
        self.symbol_table.write().unwrap().insert(
            variable_name,
            variable_type,
            next_logical_register,
        );
    }

    pub fn variable_info(&self, variable_name: &str) -> Entry {
        self.symbol_table
            .read()
            .unwrap()
            .get(variable_name)
            .unwrap()
            .clone()
    }
}

lazy_static! {
    pub static ref CONTEXT: Context = Context {
        next_logical_register: AtomicUsize::new(0),
        symbol_table: Default::default(),
    };
}
