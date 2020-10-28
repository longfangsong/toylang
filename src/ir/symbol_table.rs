use crate::ir::type_definition::TypeDefinition;
use crate::shared::data_type::Type;
use std::collections::HashMap;

pub struct SymbolTable {
    types: HashMap<String, TypeDefinition>,
    reg_types: HashMap<String, Type>,
}
