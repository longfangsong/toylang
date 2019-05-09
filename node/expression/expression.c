#include "expression.h"
#include <stdio.h>
#include <stdlib.h>

static char *get_value_string(ExpressionNode *node) {
  fprintf(stderr, "get_value_string not difined for this!");
  exit(1);
}

void init_expression_node(ExpressionNode *node, NodeType type) {
  init_ast_node((ASTNode *)node, type);
}