mod assign;

pub(crate) trait Statement {
    fn generate_ssa(&self) -> String;
}

