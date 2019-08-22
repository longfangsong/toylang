#ifndef TOYLANG_PARAMLIST_H
#define TOYLANG_PARAMLIST_H

#include "../../symbol.h"
#include "../../../../ast/node.h"

typedef struct ParamList {
    ASTNode base;
    size_t length;
    Symbol **content;
} ParamList;

ParamList *create_param_list();

void add_param(ParamList *list, Symbol *param);

#endif //TOYLANG_PARAMLIST_H
