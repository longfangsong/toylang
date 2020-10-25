use crate::ast::visitor::{ASTDisplayer, ASTVisitor};
use std::{fs::File, io::Read, path::PathBuf};
use structopt::StructOpt;

mod ast;
mod shared;

#[derive(Debug, StructOpt)]
#[structopt(name = "parser", about = "Parser command for toylang")]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    input: PathBuf,
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    let mut input_source =
        File::open(opt.input).unwrap_or_else(|_| panic!("Couldn't open input file"));
    let output_file =
        File::create(opt.output).unwrap_or_else(|_| panic!("Couldn't create output file"));
    let mut content = String::new();
    input_source
        .read_to_string(&mut content)
        .unwrap_or_else(|_| panic!("Couldn't read input file"));
    let syntax_tree = ast::from_source(&content)
        .unwrap_or_else(|_| panic!("Couldn't parse input file"))
        .1;
    let mut displayer = ASTDisplayer(output_file);
    syntax_tree.iter().map(|x| displayer.visit_ast(x)).collect()
}
