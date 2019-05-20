#include "array.h"

ArraySymbol *create_array_symbol(bool mutable, SymbolType elementType, char *name, size_t length) {
    ArraySymbol *result = malloc(sizeof(ArraySymbol));
    init_symbol((Symbol *) (result), mutable, Array, name);
    result->elementType = elementType;
    result->length = length;
    return result;
}