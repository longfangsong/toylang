#ifndef __TOYLANG__NODE__STATEMENT__IF__IF_H__
#define __TOYLANG__NODE__STATEMENT__IF__IF_H__
#include "../../expression/expression.h"
#include "../statement.h"
#include "../statementList/statementList.h"
#include <stdlib.h>
typedef struct IfNode {
  StatementNode base;
  ExpressionNode *condition;
  StatementListNode *statements;
} IfNode;

IfNode *create_if_node(ExpressionNode *condition,
                       StatementListNode *statements);

#endif
