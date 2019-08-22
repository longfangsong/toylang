#include "symbol.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "./array/array.h"

void init_symbol(Symbol *memory, bool mutable, SymbolType type, char *name) {
    memory->mutable = mutable;
    memory->type = type;
    memory->name = malloc(strlen(name) + 1);
    memory->namespace_id = (size_t) -1;
    strcpy(memory->name, name);
}

Symbol *create_symbol(bool mutable, SymbolType type, char *name) {
    Symbol *result = malloc(sizeof(Symbol));
    init_symbol(result, mutable, type, name);
    return result;
}

char *symbol_type(Symbol *symbol) {
    switch (symbol->type) {
        case Int:
            return "i32";
        case Double:
            return "double";
        case Array: {
            char *result = malloc(128);
            sprintf(result, "[%zu x %s]", ((ArraySymbol *) symbol)->length,
                    type_string(((ArraySymbol *) symbol)->elementType));
            return result;
        }
        default:
            exit(1);
    }
}
