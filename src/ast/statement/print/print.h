#ifndef TOYLANG_PRINT_H
#define TOYLANG_PRINT_H

#include "../statement.h"
#include "../../expression/expression.h"
#include "../../expression/rvalue/rvalue.h"

typedef struct PrintStatement {
    Statement base;
    RValue *expression;
} PrintStatement;

PrintStatement *create_print_statement(RValue *expression);

#endif //TOYLANG_PRINT_H
