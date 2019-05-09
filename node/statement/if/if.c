#include "if.h"
#include <stdio.h>
size_t next_if_label_id = 0;

static void generate_code(IfNode *node) {
  size_t label = next_if_label_id++;
  ((ASTNode *)(node->condition))->generate_code((ASTNode *)(node->condition));
  printf(
      "br i1 %s, label %%__if_branch_true_%zu, label %%__if_branch_false_%zu\n",
      node->condition->get_value_string(node->condition), label, label);
  printf("__if_branch_true_%zu:\n", label);
  ((StatementNode *)(node->statements))
      ->generate_code((StatementNode *)(node->statements));
  printf("br label %%__if_branch_false_%zu\n", label);
  printf("__if_branch_false_%zu:\n", label);
}

IfNode *create_if_node(ExpressionNode *condition,
                       StatementListNode *statements) {
  IfNode *result = (IfNode *)malloc(sizeof(IfNode));
  init_ast_node((ASTNode *)(result), If);
  ((ASTNode *)(result))->generate_code = (void (*)(ASTNode *))(generate_code);
  result->condition = condition;
  result->statements = statements;
  return result;
}