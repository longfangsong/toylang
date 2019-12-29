mod expression;
mod statement;

pub fn generate_ir(code: &str) -> String {
    statement::parse(&("{".to_string() + code + "}")[..]).unwrap().1.generate_ir()
}