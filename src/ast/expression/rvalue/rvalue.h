#ifndef TOYLANG_RVALUE_H
#define TOYLANG_RVALUE_H

#include "../expression.h"
#include "../../../symbolTable/symbol/type/symbolType.h"

typedef struct RValue {
    Expression base;
    SymbolType type;

    void (*generate_rvalue_code)(struct RValue *rValue);

    char *(*rvalue_ir)(struct RValue *rValue);
} RValue;

size_t next_temp_register;
#endif //TOYLANG_RVALUE_H
