use crate::symbol_table::frame::Frame;
use crate::symbol_table::symbol::Symbol;

#[derive(Debug)]
pub struct SymbolTable(Vec<Frame>);

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable(vec![Frame::new()])
    }
    pub fn push(&mut self) {
        let new_frame = Frame::new();
        self.0.push(new_frame);
    }
    pub fn pop(&mut self) {
        self.0.pop();
    }
    pub fn add_entry(&mut self, type_name: &str, symbol_name: &str) {
        let block_id = self.0.len();
        self.0.last_mut()
            .map(|frame|
                frame.insert(symbol_name.to_string(), Symbol {
                    name: symbol_name.to_string(),
                    type_name: type_name.to_string(),
                    block_id,
                }));
    }
    pub fn find_entry(&self, symbol_name: &str) -> Option<Symbol> {
        self.0.iter().rev()
            .find_map(|frame| frame.get(symbol_name).map(|it| it.clone()))
    }
}