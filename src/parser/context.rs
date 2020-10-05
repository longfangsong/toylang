use crate::ir::Register as LogicalRegister;
use crate::parser::expression::variable::Variable;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Context {
    next_logical_register: AtomicUsize,
}

impl Context {
    pub fn reset(&self) {
        self.next_logical_register.store(0, Ordering::Relaxed);
    }

    pub fn next(&self) -> LogicalRegister {
        let result = self.next_logical_register.fetch_add(1, Ordering::Relaxed);
        LogicalRegister(format!("{}", result))
    }
}

lazy_static! {
    pub static ref CONTEXT: Context = Context {
        next_logical_register: AtomicUsize::new(0)
    };
}
