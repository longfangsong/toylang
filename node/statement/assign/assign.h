#ifndef __TOYLANG__NODE__STATEMENT__ASSIGN__ASSIGN_H__
#define __TOYLANG__NODE__STATEMENT__ASSIGN__ASSIGN_H__
#include "../../expression/expression.h"
#include "../../expression/tempRegister/variable/variable.h"
#include "../statement.h"

typedef struct Assign_Node {
  StatementNode base;
  VariableNode *variable;
  ExpressionNode *value;
} AssignNode;

AssignNode *create_assign_node(VariableNode *variable, ExpressionNode *value);

#endif
