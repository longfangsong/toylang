#ifndef TOYLANG_IF_H
#define TOYLANG_IF_H

#include "../statement.h"
#include "../../expression/expression.h"
#include "../compound/compound.h"
#include "../../expression/rvalue/rvalue.h"

typedef struct IfStatement {
    Statement base;
    RValue *condition;
    CompoundStatement *statement;
    CompoundStatement *else_statement;
} IfStatement;

IfStatement *create_if_statement(RValue *condition,
                                 CompoundStatement *statement,
                                 CompoundStatement *else_statement
);

#endif //TOYLANG_IF_H
