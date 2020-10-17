use crate::ir::utils::LocalOrNumberLiteral;

pub struct Call {
    result: String,
    name: String,
    params: Vec<LocalOrNumberLiteral>,
}
