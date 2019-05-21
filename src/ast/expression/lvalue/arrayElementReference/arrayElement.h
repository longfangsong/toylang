
#ifndef TOYLANG_ARRAYELEMENT_H
#define TOYLANG_ARRAYELEMENT_H

#include "../lvalue.h"
#include "../../../../symbolTable/symbol/array/array.h"

typedef struct ArrayElementReference {
    LValue base;
    ArraySymbol *array;
    size_t index;
    size_t pointer_register_id;
} ArrayElementReference;

ArrayElementReference *create_array_element_reference(ArraySymbol *array, size_t index);

#endif //TOYLANG_ARRAYELEMENT_H
