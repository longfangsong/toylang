use crate::ir::TypeDefinition;
use crate::shared;

pub enum Type {
    Integer(shared::data_type::Integer),
    Struct(Vec<shared::data_type::Integer>),
    None,
    Address,
}

impl Type {
    // in bytes
    pub fn size(&self) -> usize {
        match self {
            Type::Integer(x) => x.width / 8,
            Type::Struct(x) => x.iter().map(Type::size).sum(),
            Type::None => 0,
            Type::Address => 4,
        }
    }
}
