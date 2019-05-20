#include "while.h"
#include <stdio.h>

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


WhileStatement *create_while_statement(Expression *condition, CompoundStatement *statement) {
    WhileStatement *result = (WhileStatement *) malloc(sizeof(WhileStatement));
#ifdef DEBUG
    ((ASTNode *) (result))->print_ast_node = (void (*)(ASTNode *, size_t)) print_ast_node;
#endif
    ((ASTNode *) (result))->free_node = (void (*)(ASTNode *)) free_node;
    result->condition = condition;
    result->statement = statement;
    return result;
}