mod ast;
pub mod backend;
pub mod utility;

fn main() {
    let ast = ast::from_source(
        r#"fn f(a: i32) -> i32 {
        let b: i32 = 1;
        let c: i32 = a + b;
        return c;
    }"#,
    )
    .unwrap()
    .1;
    let result = backend::riscv::compile(&ast);
    println!("{}", result);
}
