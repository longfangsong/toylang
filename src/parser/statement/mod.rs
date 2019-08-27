pub mod variable_declaration;
pub mod assignment;

pub trait Statement {
    fn generate_code(self) -> String;
}