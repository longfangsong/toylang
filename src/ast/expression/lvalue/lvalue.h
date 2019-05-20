#ifndef TOYLANG_LVALUE_H
#define TOYLANG_LVALUE_H

#include "../expression.h"
#include "../rvalue/rvalue.h"

typedef struct LValue {
    RValue base;

    char *(*lvalue_ir)(struct LValue *);

    void (*generate_lvalue_code)(struct LValue *);
} LValue;
#endif //TOYLANG_LVALUE_H
