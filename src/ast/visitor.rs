use crate::ast::{
    function::{FunctionDefinition, FunctionDefinitionVisitor},
    global_definition::{VariableDefinition, VariableDefinitionVisitor},
    statement::{
        assign::{Assign, AssignVisitor},
        declare::{Declare, DeclareVisitor},
        function_call::{FunctionCall, FunctionCallVisitor},
        if_statement::{If, IfVisitor},
        return_statement::{Return, ReturnVisitor},
        while_statement::{While, WhileVisitor},
        StatementVisitor,
    },
    type_definition::{TypeDefinition, TypeDefinitionVisitor},
    ASTNode,
};
use std::io::Write;

pub trait ASTVisitor:
    TypeDefinitionVisitor + VariableDefinitionVisitor + FunctionDefinitionVisitor
{
    fn visit_ast(&mut self, ast: &ASTNode) {
        match ast {
            ASTNode::Type(type_definition) => self.visit_type_definition(type_definition),
            ASTNode::Function(function_definition) => {
                self.visit_function_definition(function_definition)
            }
            ASTNode::GlobalVariable(variable_definition) => {
                self.visit_global_definition(variable_definition)
            }
        }
    }
}

pub struct ASTDisplayer<W: Write>(pub W);

impl<W: Write> TypeDefinitionVisitor for ASTDisplayer<W> {
    fn visit_type_definition(&mut self, type_definition: &TypeDefinition) {
        writeln!(self.0, "{:#?}", type_definition).unwrap()
    }
}

impl<W: Write> VariableDefinitionVisitor for ASTDisplayer<W> {
    fn visit_global_definition(&mut self, variable_definition: &VariableDefinition) {
        writeln!(self.0, "{:#?}", variable_definition).unwrap()
    }
}

impl<W: Write> DeclareVisitor for ASTDisplayer<W> {
    fn visit_declare(&mut self, declare: &Declare) {
        writeln!(self.0, "{:#?}", declare).unwrap()
    }
}

impl<W: Write> AssignVisitor for ASTDisplayer<W> {
    fn visit_assign(&mut self, assign: &Assign) {
        writeln!(self.0, "{:#?}", assign).unwrap()
    }
}

impl<W: Write> ReturnVisitor for ASTDisplayer<W> {
    fn visit_return(&mut self, return_statement: &Return) {
        writeln!(self.0, "{:#?}", return_statement).unwrap()
    }
}

impl<W: Write> IfVisitor for ASTDisplayer<W> {
    fn visit_if(&mut self, if_statement: &If) {
        writeln!(self.0, "{:#?}", if_statement).unwrap()
    }
}

impl<W: Write> WhileVisitor for ASTDisplayer<W> {
    fn visit_while(&mut self, while_statement: &While) {
        writeln!(self.0, "{:#?}", while_statement).unwrap()
    }
}

impl<W: Write> FunctionCallVisitor for ASTDisplayer<W> {
    fn visit_function_call(&mut self, function_call: &FunctionCall) {
        writeln!(self.0, "{:#?}", function_call).unwrap()
    }
}

impl<W: Write> StatementVisitor for ASTDisplayer<W> {}

impl<W: Write> FunctionDefinitionVisitor for ASTDisplayer<W> {
    fn visit_function_definition(&mut self, function_definition: &FunctionDefinition) {
        writeln!(self.0, "{:#?}", function_definition).unwrap()
    }
}

impl<W: Write> ASTVisitor for ASTDisplayer<W> {}
