#ifndef TOYLANG_WHILE_H
#define TOYLANG_WHILE_H

#include "../statement.h"
#include "../../expression/expression.h"
#include "../compound/compound.h"

typedef struct WhileStatement {
    Statement base;
    Expression *condition;
    CompoundStatement *statement;
} WhileStatement;

WhileStatement *create_while_statement(Expression *condition, CompoundStatement *statement);

#endif //TOYLANG_WHILE_H
