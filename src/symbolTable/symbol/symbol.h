#ifndef TOYLANG_SYMBOL_H
#define TOYLANG_SYMBOL_H

#include "type/symbolType.h"
#include <stdbool.h>
#include <stdlib.h>

typedef struct {
    SymbolType type;
    bool mutable;
    char *name;
    size_t namespace_id;
} Symbol;

void init_symbol(Symbol *memory, bool mutable, SymbolType type, char *name);

Symbol *create_symbol(bool mutable, SymbolType type, char *name);

char *type_name(Symbol *symbol);

#endif //TOYLANG_SYMBOL_H
