#ifndef TOYLANG_SYMBOLTYOE_H
#define TOYLANG_SYMBOLTYOE_H

#include "symbol/symbol.h"
#include <stdlib.h>

typedef struct {
    size_t length;
    Symbol **symbols;
    size_t namespace_id;
} SymbolTableFrame;

typedef struct {
    size_t length;
    SymbolTableFrame **frames;
} SymbolTableStack;

void push_frame();

void pop_frame();

void add_symbol(Symbol *symbol);

Symbol *get_symbol(char *name);

#endif //TOYLANG_SYMBOLTYPE_H
