#ifndef TOYLANG_ASSIGN_H
#define TOYLANG_ASSIGN_H

#include "../statement.h"
#include "../../expression/lvalue/lvalue.h"

typedef struct AssignStatement {
    Statement base;
    LValue *lhs;
    RValue *rhs;
} AssignStatement;

AssignStatement *create_assign_statement(LValue *lhs, RValue *rhs);

#endif //TOYLANG_ASSIGN_H
