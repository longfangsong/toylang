use crate::ir::utils::LocalOrGlobal;
use crate::shared::data_type::Type;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Load {
    pub to_register: String,
    pub data_type: Type,
    pub from_register: LocalOrGlobal,
}
