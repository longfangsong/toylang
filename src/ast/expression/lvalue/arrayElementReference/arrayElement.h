
#ifndef TOYLANG_ARRAYELEMENT_H
#define TOYLANG_ARRAYELEMENT_H

#include "../lvalue.h"
#include "../../../../symbolTable/symbol/array/array.h"

typedef struct ArrayElementReference {
    LValue base;
    ArraySymbol *array;
    RValue *index;
    size_t rvalue_register_id;
    size_t lvalue_register_id;
} ArrayElementReference;

ArrayElementReference *create_array_element_reference(ArraySymbol *array, RValue *index);

#endif //TOYLANG_ARRAYELEMENT_H
