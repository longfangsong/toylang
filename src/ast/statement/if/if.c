#include "if.h"
#include <stdio.h>

size_t next_if_id = 0;
#ifdef DEBUG

static void print_ast_node(IfStatement *ifStatement, size_t layer) {
    for (size_t i = 0; i < layer; ++i) {
        printf("  ");
    }
    printf("IfStatement\n");
    for (size_t i = 0; i < layer; ++i) {
        printf("  ");
    }
    printf("Condition:\n");
    ((ASTNode *) (ifStatement->condition))->print_ast_node(((ASTNode *) (ifStatement->condition)), layer + 1);
    for (size_t i = 0; i < layer; ++i) {
        printf("  ");
    }
    printf("Statement:\n");
    ((ASTNode *) (ifStatement->statement))->print_ast_node((ASTNode *) (ifStatement->statement), layer + 1);
    if (ifStatement->else_statement != NULL) {
        for (size_t i = 0; i < layer; ++i) {
            printf("  ");
        }
        printf("ElseStatement:\n");
        ((ASTNode *) (ifStatement->else_statement))->print_ast_node((ASTNode *) (ifStatement->else_statement),
                                                                    layer + 1);
    }
}

#endif

static void free_node(IfStatement *ifStatement) {
    ((ASTNode *) (ifStatement->condition))->free_node((ASTNode *) (ifStatement->condition));
    ((ASTNode *) (ifStatement->statement))->free_node((ASTNode *) (ifStatement->statement));
    if (ifStatement->else_statement != NULL) {
        ((ASTNode *) (ifStatement->else_statement))->free_node((ASTNode *) (ifStatement->else_statement));
    }
    free(ifStatement);
}

static void generate_code(IfStatement *node) {
    ((RValue *) (node->condition))->generate_rvalue_code((RValue *) (node->condition));
    char *condition_rvalue_ir = node->condition->rvalue_ir(node->condition);
    size_t if_id = next_if_id++;
    printf("br i1 %s, "
           "label %%if_lable_%zu,"
           "label %%else_lable_%zu\n",
           condition_rvalue_ir,
           if_id,
           if_id);
    printf("if_lable_%zu:\n", if_id);
    ((Statement *) (node->statement))->generate_code((Statement *) (node->statement));
    printf("br label %%if_end_lable_%zu\n", if_id);
    printf("else_lable_%zu:\n", if_id);
    if (node->else_statement != NULL) {
        ((Statement *) (node->else_statement))->generate_code((Statement *) (node->else_statement));
    }
    printf("br label %%if_end_lable_%zu\n", if_id);
    printf("if_end_lable_%zu:\n", if_id);
    free(condition_rvalue_ir);
}

IfStatement *create_if_statement(
        RValue *condition,
        CompoundStatement *statement,
        CompoundStatement *else_statement
) {
    IfStatement *result = (IfStatement *) malloc(sizeof(IfStatement));
#ifdef DEBUG
    ((ASTNode *) (result))->print_ast_node = (void (*)(ASTNode *, size_t)) print_ast_node;
#endif
    ((ASTNode *) (result))->free_node = (void (*)(ASTNode *)) free_node;
    result->condition = condition;
    result->statement = statement;
    result->else_statement = else_statement;
    ((Statement *) (result))->generate_code = (void (*)(ASTNode *)) generate_code;
    return result;
}