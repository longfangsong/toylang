use crate::ir::basic_block::BasicBlock;
pub use crate::ir::basic_block::BasicBlockVisitor;
use crate::ir::function::{FunctionDefinition, FunctionDefinitionVisitor};
use crate::ir::global_definition::{GlobalDefinition, GlobalDefinitionVisitor};
pub use crate::ir::statements::{IRStatementVisitor, TerminatorVisitor};
use crate::ir::type_definition::{TypeDefinition, TypeDefinitionVisitor};
use crate::ir::IR;
use std::io::Write;

pub trait IRVisitor:
    TypeDefinitionVisitor + FunctionDefinitionVisitor + GlobalDefinitionVisitor
{
    fn visit_ir(&mut self, ir: &IR) {
        match ir {
            IR::TypeDefinition(type_definition) => self.visit_type_definition(type_definition),
            IR::FunctionDefinition(function_definition) => {
                self.visit_function_definition(function_definition)
            }
            IR::GlobalDefinition(global_definition) => {
                self.visit_global_definition(global_definition)
            }
        }
    }
}

pub struct IRDisplayer<W: Write>(pub W);

impl<W: Write> BasicBlockVisitor for IRDisplayer<W> {
    fn visit_basic_block(&mut self, basic_block: &BasicBlock) {
        unimplemented!()
    }
}

impl<W: Write> FunctionDefinitionVisitor for IRDisplayer<W> {
    fn visit_function_definition(&mut self, function_definition: &FunctionDefinition) {
        writeln!(self.0, "{:?}", function_definition).unwrap();
    }
}

impl<W: Write> GlobalDefinitionVisitor for IRDisplayer<W> {
    fn visit_global_definition(&mut self, global_definition: &GlobalDefinition) {
        writeln!(self.0, "{:?}", global_definition).unwrap();
    }
}

impl<W: Write> TypeDefinitionVisitor for IRDisplayer<W> {
    fn visit_type_definition(&mut self, type_definition: &TypeDefinition) {
        writeln!(self.0, "{:?}", type_definition).unwrap();
    }
}

impl<W: Write> IRVisitor for IRDisplayer<W> {}
