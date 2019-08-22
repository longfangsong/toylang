#ifndef TOYLANG_RETURN_H
#define TOYLANG_RETURN_H

#include "../../expression/rvalue/rvalue.h"
#include "../statement.h"

typedef struct ReturnStatement {
    Statement base;
    RValue *expression;
} ReturnStatement;

ReturnStatement *create_return_statement(RValue *expression);

#endif //TOYLANG_RETURN_H
