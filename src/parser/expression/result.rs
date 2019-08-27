#[derive(Debug)]
pub struct ExpressionParseResult {
    pub type_name: String,
    pub generated_code: String,
    pub bind_to: String,
}