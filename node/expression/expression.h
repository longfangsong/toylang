#ifndef __TOYLANG__NODE__EXPRESSION__EXPRESSION_H__
#define __TOYLANG__NODE__EXPRESSION__EXPRESSION_H__
#include "../node.h"

typedef struct Expression_Node {
  ASTNode base;
  char *(*get_value_string)(struct Expression_Node *node);
} ExpressionNode;

void init_expression_node(ExpressionNode *node, NodeType type);

#endif
