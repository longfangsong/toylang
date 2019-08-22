#ifndef TOYLANG_SYMBOL_FUNCTION_H
#define TOYLANG_SYMBOL_FUNCTION_H

#include "../symbol.h"
#include "paramList/paramList.h"
#include "../../../ast/statement/compound/compound.h"

typedef struct Function {
    Symbol base;
    ParamList *params;
    CompoundStatement *statement;
} Function;

Function *create_function_symbol(SymbolType returnType, char *name, ParamList *params, CompoundStatement *statement);

#endif //TOYLANG_FUNCTION_H
