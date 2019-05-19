#ifndef TOYLANG_DECLARE_H
#define TOYLANG_DECLARE_H

#include "../statement.h"
#include "../../../symbolTable/symbol/symbol.h"
#include "../assign/assign.h"

typedef struct DeclareStatement {
    Statement base;
    Symbol *variable;
    AssignStatement *initial;
} DeclareStatement;

DeclareStatement *create_declare_statement(Symbol *variable, AssignStatement *initial);

#endif //TOYLANG_DECLARE_H
