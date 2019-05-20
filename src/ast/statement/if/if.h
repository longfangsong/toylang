#ifndef TOYLANG_IF_H
#define TOYLANG_IF_H

#include "../statement.h"
#include "../../expression/expression.h"
#include "../compound/compound.h"

typedef struct IfStatement {
    Statement base;
    Expression *condition;
    CompoundStatement *statement;
    CompoundStatement *else_statement;
} IfStatement;

IfStatement *create_if_statement(Expression *condition,
                                 CompoundStatement *statement,
                                 CompoundStatement *else_statement
);

#endif //TOYLANG_IF_H
