#include "assign.h"
#include <stdio.h>

static void generate_code(AssignNode *node) {
  ((ASTNode *)(node->value))->generate_code((ASTNode *)(node->value));
  printf("store i32 %s, i32* %%%s\n",
         ((ExpressionNode *)(node->value))
             ->get_value_string((ExpressionNode *)(node->value)),
         node->variable->name);
}

AssignNode *create_assign_node(VariableNode *variable, ExpressionNode *value) {
  AssignNode *result = (AssignNode *)malloc(sizeof(AssignNode));
  init_ast_node((ASTNode *)(result), Assign);
  ((ASTNode *)(result))->generate_code = (void (*)(ASTNode *))(generate_code);
  result->value = value;
  result->variable = variable;
  return result;
}