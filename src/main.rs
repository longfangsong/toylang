#[macro_use]
extern crate lazy_static;

use std::env;
use std::fs::File;
use std::io::{Read, Write};

use crate::code::generate_ir;
use crate::register::assign::generate_assign_map;
use crate::register::SSARegister;

#[macro_use]
mod tools;
mod code;
mod ssa;
mod register;

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
    let result = env::args().skip(1)
        .find(|it| !it.starts_with('-'))
        .and_then(|path| {
            read_file(path).ok()
        })
        .map(|content| {
            let ir = generate_ir(&content);
            let require_registers: Vec<SSARegister> = ir.iter().map(|it| it.require_registers()).flatten().collect();
            let assign_map = generate_assign_map(&require_registers);
            ir.iter()
                .map(|it| it.generate_asm(&assign_map))
                .collect::<Vec<_>>().join("")
        });
    println!("{}", result.unwrap());
    Ok(())
}
