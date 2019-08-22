#ifndef TOYLANG_VARIABLE_H
#define TOYLANG_VARIABLE_H

#include "../../statement.h"
#include "../../../../symbolTable/symbol/symbol.h"
#include "../../assign/assign.h"

typedef struct VariableDeclareStatement {
    Statement base;
    Symbol *variable;
    AssignStatement *initial;
} VariableDeclareStatement;

VariableDeclareStatement *create_variable_declare_statement(Symbol *variable, AssignStatement *initial);

#endif //TOYLANG_VARIABLE_H
