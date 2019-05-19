#ifndef TOYLANG_ARRAY_H
#define TOYLANG_ARRAY_H

#include <stdlib.h>
#include "../symbol.h"

typedef struct {
    Symbol base;
    SymbolType elementType;
    size_t length;
} ArraySymbol;

ArraySymbol *create_array_symbol(bool mutable, SymbolType elementType, char *name, size_t length);

#endif //TOYLANG_ARRAY_H
