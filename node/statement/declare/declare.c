#include "declare.h"
#include <stdio.h>

static void generate_code(DeclareNode *node) {
  printf("%%%s = alloca i32\n", node->variable->name);
}

DeclareNode *create_declare_node(VariableNode *variable) {
  DeclareNode *result = (DeclareNode *)malloc(sizeof(DeclareNode));
  init_ast_node((ASTNode *)(result), Declare);
  ((ASTNode *)(result))->generate_code = (void (*)(ASTNode *))(generate_code);
  result->variable = variable;
  return result;
}
