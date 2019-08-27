#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Symbol {
    pub name: String,
    pub type_name: String,
    pub block_id: usize,
}

impl Symbol {
    pub fn declaration_name(&self) -> String {
        format!("%{}_block{}", self.name, self.block_id)
    }
}

impl Clone for Symbol {
    fn clone(&self) -> Self {
        Symbol {
            name: self.name.clone(),
            type_name: self.type_name.clone(),
            block_id: self.block_id,
        }
    }
}

