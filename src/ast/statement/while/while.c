#include "while.h"
#include <stdio.h>

size_t next_while_id = 0;
#ifdef DEBUG

static void print_ast_node(WhileStatement *whileStatement, size_t layer) {
    for (size_t i = 0; i < layer; ++i) {
        printf("  ");
    }
    printf("WhileStatement\n");
    for (size_t i = 0; i < layer; ++i) {
        printf("  ");
    }
    printf("Condition:\n");
    ((ASTNode *) (whileStatement->condition))->print_ast_node(((ASTNode *) (whileStatement->condition)), layer + 1);
    for (size_t i = 0; i < layer; ++i) {
        printf("  ");
    }
    printf("Statement:\n");
    ((ASTNode *) (whileStatement->statement))->print_ast_node((ASTNode *) (whileStatement->statement), layer + 1);
}

#endif

static void free_node(WhileStatement *whileStatement) {
    ((ASTNode *) (whileStatement->condition))->free_node((ASTNode *) (whileStatement->condition));
    ((ASTNode *) (whileStatement->statement))->free_node((ASTNode *) (whileStatement->statement));
    free(whileStatement);
}

static void generate_code(WhileStatement *node) {
    size_t while_id = next_while_id++;
    printf("br label %%while_condition_%zu\n", while_id);
    printf("while_condition_%zu:\n", while_id);
    ((RValue *) (node->condition))->generate_rvalue_code((RValue *) (node->condition));
    char *condition_rvalue_ir = node->condition->rvalue_ir(node->condition);
    printf("br i1 %s, "
           "label %%while_body_%zu,"
           "label %%while_end_%zu\n",
           condition_rvalue_ir,
           while_id,
           while_id
    );
    printf("while_body_%zu:\n", while_id);
    ((Statement *) (node->statement))->generate_code((Statement *) (node->statement));
    printf("br label %%while_condition_%zu\n", while_id);
    printf("while_end_%zu:\n", while_id);
    free(condition_rvalue_ir);
}

WhileStatement *create_while_statement(RValue *condition, CompoundStatement *statement) {
    WhileStatement *result = (WhileStatement *) malloc(sizeof(WhileStatement));
#ifdef DEBUG
    ((ASTNode *) (result))->print_ast_node = (void (*)(ASTNode *, size_t)) print_ast_node;
#endif
    ((ASTNode *) (result))->free_node = (void (*)(ASTNode *)) free_node;
    ((Statement *) (result))->generate_code = (void (*)(Statement *)) generate_code;
    result->condition = condition;
    result->statement = statement;
    return result;
}