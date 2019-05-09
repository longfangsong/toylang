#include "tempRegister.h"
#include <stdio.h>

size_t next_id = 0;

static char *get_value_string(TempRegisterNode *node) {
  char *buffer = malloc(128);
  sprintf(buffer, "%%temp%zu", node->register_id);
  return buffer;
}

void init_register_node(TempRegisterNode *node, NodeType type) {
  init_expression_node((ExpressionNode *)(node), type);
  ((ExpressionNode *)(node))->get_value_string =
      (char *(*)(ExpressionNode *))(get_value_string);
  node->register_id = next_id++;
}