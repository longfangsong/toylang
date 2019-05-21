#ifndef TOYLANG_WHILE_H
#define TOYLANG_WHILE_H

#include "../statement.h"
#include "../../expression/expression.h"
#include "../compound/compound.h"
#include "../../expression/rvalue/rvalue.h"

typedef struct WhileStatement {
    Statement base;
    RValue *condition;
    CompoundStatement *statement;
} WhileStatement;

WhileStatement *create_while_statement(RValue *condition, CompoundStatement *statement);

#endif //TOYLANG_WHILE_H
