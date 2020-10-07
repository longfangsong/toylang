use crate::ir::Register as LogicalRegister;
use crate::shared::data_type::Integer;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Entry {
    data_type: Integer,
    assigned_register: LogicalRegister,
}

#[derive(Debug, Default)]
pub struct SymbolTable(HashMap<String, Entry>);

impl SymbolTable {
    pub fn insert(&mut self, name: &str, data_type: Integer, assigned_register: LogicalRegister) {
        self.0.insert(name.to_string(), Entry { data_type, assigned_register });
    }

    pub fn get(&self, name: &str) -> Option<&Entry> {
        self.0.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Entry> {
        self.0.get_mut(name)
    }
}
