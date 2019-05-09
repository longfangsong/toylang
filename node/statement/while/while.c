#include "while.h"
#include <stdio.h>

size_t next_while_label_id = 0;

static void generate_code(WhileNode *node) {
    size_t label = next_while_label_id++;
    printf("br label %%__while_condition_%zu\n", label);
    printf("__while_condition_%zu:\n", label);
    ((ASTNode *) (node->condition))->generate_code((ASTNode *) (node->condition));
    printf("br i1 %s, label %%__while_body_%zu, label %%__while_end_%zu\n",
           node->condition->get_value_string(node->condition),
           label, label);
    printf("__while_body_%zu:\n", label);
    ((StatementNode *) (node->statements))
            ->generate_code((StatementNode *) (node->statements));
    printf("br label %%__while_condition_%zu\n", label);
    printf("__while_end_%zu:\n", label);
}

WhileNode *create_while_node(ExpressionNode *condition,
                             StatementListNode *statements) {
    WhileNode *result = (WhileNode *) malloc(sizeof(WhileNode));
    init_ast_node((ASTNode *) (result), While);
    ((ASTNode *) (result))->generate_code = (void (*)(ASTNode *)) (generate_code);
    result->condition = condition;
    result->statements = statements;
    return result;
}