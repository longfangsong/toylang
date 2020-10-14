mod symbol_table;

use crate::ast::context::symbol_table::{Entry, SymbolTable};
use crate::ast::expression::variable_ref::VariableRef;
use crate::ir::Register as LogicalRegister;
use crate::shared::data_type::Integer;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::RwLock;

pub struct Context {
    next_logical_register: AtomicUsize,
    next_branch_id: AtomicUsize,
    symbol_table: RwLock<SymbolTable>,
}

impl Context {
    pub fn reset(&self) {
        self.next_logical_register.store(0, Ordering::Relaxed);
    }

    pub fn next(&self, data_type: Integer) -> LogicalRegister {
        let result = self.next_logical_register.fetch_add(1, Ordering::Relaxed);
        LogicalRegister {
            name: format!("{}", result),
            data_type: data_type.into(),
        }
    }

    pub fn next_branch_id(&self) -> String {
        format!(
            "branch_{}",
            self.next_branch_id.fetch_add(1, Ordering::Relaxed)
        )
    }

    pub fn insert_variable(&self, variable_name: &str, variable_type: Integer) {
        let next_logical_register = self.next(variable_type.clone());
        self.symbol_table
            .write()
            .unwrap()
            .insert(variable_name, next_logical_register);
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
        next_branch_id: AtomicUsize::new(0),
        symbol_table: Default::default(),
    };
}
