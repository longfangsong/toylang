#ifndef __TOYLANG__NODE__STATEMENT__WHILE__WHILE_H__
#define __TOYLANG__NODE__STATEMENT__WHILE__WHILE_H__

#include "../../expression/expression.h"
#include "../statement.h"
#include "../statementList/statementList.h"

#include <stdlib.h>

typedef struct {
    StatementNode base;
    ExpressionNode *condition;
    StatementListNode *statements;
} WhileNode;

WhileNode *create_while_node(ExpressionNode *condition,
                             StatementListNode *statements);

#endif
