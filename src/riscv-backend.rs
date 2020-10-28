use crate::ir::visitor::{BasicBlockVisitor, IRVisitor};
use crate::ir::{
    BasicBlock, FunctionDefinition, FunctionDefinitionVisitor, GlobalDefinition,
    GlobalDefinitionVisitor, IRStatementVisitor, TerminatorVisitor, TypeDefinition,
    TypeDefinitionVisitor,
};
use crate::riscv::data_type;
use crate::shared::data_type::Type;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;

mod ir;
mod riscv;
mod shared;

#[derive(Debug, StructOpt)]
#[structopt(name = "riscv backend", about = "translate ir into riscv asm")]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    input: PathBuf,
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,
}

enum RegisterMap {
    RealRegister(String),
    SpilledToStack(usize),
    StackAllocaRef(usize),
}

struct CodeGenerator {
    types: HashMap<String, data_type::Type>,
    global_variable_asm: Vec<String>,
}

impl TypeDefinitionVisitor for CodeGenerator {
    fn visit_type_definition(&mut self, type_definition: &TypeDefinition) {
        let fields: Vec<_> = type_definition
            .fields
            .iter()
            .map(|it| {
                if let Some(Type::Integer(it)) = it {
                    it.clone()
                } else {
                    unimplemented!()
                }
            })
            .collect();
        let new_type = data_type::Type::Struct(fields);
        self.types.insert(type_definition.name.clone(), new_type);
    }
}

impl GlobalDefinitionVisitor for CodeGenerator {
    fn visit_global_definition(&mut self, global_definition: &GlobalDefinition) {
        self.global_variable_asm.push(format!(
            "{}:\n    .word {}",
            global_definition.item.0, global_definition.initial_value.0
        ))
    }
}

struct RegisterAssigner {}

impl IRStatementVisitor for RegisterAssigner {
    fn visit_ir_statement(&mut self) {}
}

impl IRStatementVisitor for CodeGenerator {
    fn visit_ir_statement(&mut self) {}
}

impl TerminatorVisitor for CodeGenerator {
    fn visit_terminator(&mut self) {
        unimplemented!()
    }
}

impl BasicBlockVisitor for CodeGenerator {
    fn visit_basic_block(&mut self, basic_block: &BasicBlock) {}
}

impl FunctionDefinitionVisitor for CodeGenerator {
    fn visit_function_definition(&mut self, function_definition: &FunctionDefinition) {
        unimplemented!()
    }
}

impl IRVisitor for CodeGenerator {}

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
    let irs = ir::from_source(&content)
        .unwrap_or_else(|_| panic!("Couldn't parse input file"))
        .1;
}
