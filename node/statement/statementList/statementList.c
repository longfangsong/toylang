#include "statementList.h"

static void generate_code(StatementListNode *node) {
  for (size_t i = 0; i < node->size; ++i) {
    ((ASTNode *)(node->statements[i]))
        ->generate_code((ASTNode *)(node->statements[i]));
  }
}

StatementListNode *create_list_node() {
  StatementListNode *result =
      (StatementListNode *)malloc(sizeof(StatementListNode));
  init_ast_node((ASTNode *)(result), StatementGroup);
  ((ASTNode *)(result))->generate_code = (void (*)(ASTNode *))(generate_code);
  result->size = 0;
  result->statements = NULL;
  return result;
}

void push(StatementListNode *node, StatementNode *other) {
  node->statements = (StatementNode **)realloc(
      node->statements, sizeof(StatementNode *) * (node->size + 1));
  node->statements[node->size] = other;
  ++node->size;
}