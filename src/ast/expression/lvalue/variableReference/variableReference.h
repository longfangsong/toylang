#ifndef TOYLANG_VARIABLEREFERENCE_H
#define TOYLANG_VARIABLEREFERENCE_H

#include "../lvalue.h"
#include "../../../../symbolTable/symbol/symbol.h"

typedef struct VariableReference {
    LValue base;
    Symbol *variable;
} VariableReference;

VariableReference *create_variable_reference(Symbol *variable);

#endif //TOYLANG_VARIABLEREFERENCE_H
