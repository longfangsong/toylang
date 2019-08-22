#include "function.h"

Function *create_function_symbol(SymbolType returnType, char *name, ParamList *params, CompoundStatement *statement) {
    Function *result = (Function *) malloc(sizeof(Function));
    ((Symbol *) result)->mutable = false;
    ((Symbol *) result)->type = returnType;
    ((Symbol *) result)->name = name;
    result->params = params;
    result->statement = statement;
    return result;
}
