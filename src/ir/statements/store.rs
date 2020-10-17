use crate::ir::utils::{LocalOrGlobal, LocalOrNumberLiteral};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Store {
    pub data_type: String,
    pub source: LocalOrNumberLiteral,
    pub target: LocalOrGlobal,
}
