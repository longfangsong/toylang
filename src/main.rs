use std::env;
use std::fs::File;
use std::io::{Read, Write};

use crate::code_generator::{assign_registers, parse_ir};
use crate::parser::generate_ir;

#[macro_use]
mod tools;
mod parser;
mod code_generator;

fn read_file(path: String) -> std::io::Result<String> {
    let mut content = String::new();
    let mut f = File::open(path)?;
    f.read_to_string(&mut content)?;
    Ok(content)
}

fn write_file(file: &mut File, content: String) {
    file.write_all(&content.as_bytes()[..]).unwrap_or_else(|_| panic!("write file failed"));
}

fn main() -> std::io::Result<()> {
    let ir = env::args().skip(1)
        .find(|it| !it.starts_with('-'))
        .and_then(|path| {
            read_file(path).ok()
        })
        .map(|source| generate_ir(&source[..]));
    let mut f = File::create("ir.tir")?;
    if let Some(content) = ir.clone() {
        write_file(&mut f, content)
    }
    let ir = ir.unwrap();
    let result = parse_ir(&ir[..]).unwrap().1;
    let final_ir = assign_registers(&result);
    println!("{}", final_ir.iter()
        .map(|it| it.generate_asm())
        .collect::<Vec<_>>().join("\n"));
    Ok(())
}
