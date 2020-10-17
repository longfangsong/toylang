#[derive(Debug, Eq, PartialEq, Clone)]
pub enum LocalOrNumberLiteral {
    Local(String),
    NumberLiteral(i64),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum LocalOrGlobal {
    Local(String),
    Global(String),
}
