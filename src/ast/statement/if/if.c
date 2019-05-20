#include "if.h"
#include <stdio.h>

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


IfStatement *create_if_statement(
        Expression *condition,
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
    return result;
}