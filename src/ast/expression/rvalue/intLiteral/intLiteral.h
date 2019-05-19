#ifndef TOYLANG_CONSTANT_H
#define TOYLANG_CONSTANT_H

#include "../rvalue.h"

typedef struct IntLiteral {
    RValue base;
    int value;
} IntLiteral;

IntLiteral *create_int_literal(int value);

#endif //TOYLANG_CONSTANT_H
