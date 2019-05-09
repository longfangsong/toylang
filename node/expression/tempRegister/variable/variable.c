#include "variable.h"
#include <stdio.h>
#include <string.h>
static void generate_code(VariableNode *node) {
  printf("%s = load i32, i32* %%%s\n",
         ((ExpressionNode *)node)->get_value_string((ExpressionNode *)node),
         node->name);
}

VariableNode *create_variable_node(char *name) {
  VariableNode *result = (VariableNode *)malloc(sizeof(VariableNode));
  init_register_node((TempRegisterNode *)(result), Variable);
  ((ASTNode *)result)->generate_code = (void (*)(ASTNode *))(generate_code);
  result->name = (char *)malloc(strlen(name) + 1);
  strcpy(result->name, name);
  return result;
}