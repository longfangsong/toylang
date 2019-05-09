#include "constant.h"
#include <stdio.h>
#include <stdlib.h>
char buffer[128];

static char *get_value_string(ConstantNode *node) {
  sprintf(buffer, "%d", node->value);
  return buffer;
}

static void generate_code(ConstantNode *node) {}

ConstantNode *create_constant_node(int value) {
  ConstantNode *result = (ConstantNode *)malloc(sizeof(ConstantNode));
  init_expression_node((ExpressionNode *)(result), Const);
  ((ASTNode *)(result))->generate_code = (void (*)(ASTNode *))generate_code;
  ((ExpressionNode *)(result))->get_value_string =
      (char *(*)(ExpressionNode *))(get_value_string);
  result->value = value;
  return result;
}