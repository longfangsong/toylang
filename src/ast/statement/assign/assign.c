#include "assign.h"
#include "../../expression/lvalue/variableReference/variableReference.h"
#include <stdio.h>

#ifdef DEBUG

static void print_ast_node(AssignStatement *node, size_t layer) {
    for (size_t i = 0; i < layer; ++i) {
        printf("  ");
    }
    printf("AssignStatement\n");
    for (size_t i = 0; i < layer; ++i) {
        printf("  ");
    }
    printf("lhs:\n");
    ((ASTNode *) (node->lhs))->print_ast_node((ASTNode *) (node->lhs), layer + 1);
    for (size_t i = 0; i < layer; ++i) {
        printf("  ");
    }
    printf("rhs:\n");
    ((ASTNode *) (node->rhs))->print_ast_node((ASTNode *) (node->rhs), layer + 1);
    printf("\n");
}

#endif

static void free_node(AssignStatement *node) {
    ((ASTNode *) (node->lhs))->free_node((ASTNode *) (node->lhs));
    ((ASTNode *) (node->rhs))->free_node((ASTNode *) (node->rhs));
    free(node);
}

AssignStatement *create_assign_statement(LValue *lhs, RValue *rhs) {
    AssignStatement *result = (AssignStatement *) malloc(sizeof(AssignStatement));
#ifdef DEBUG
    ((ASTNode *) result)->print_ast_node = (void (*)(ASTNode *, size_t)) print_ast_node;
#endif
    ((ASTNode *) result)->free_node = (void (*)(ASTNode *)) free_node;
    result->lhs = lhs;
    result->rhs = rhs;
    return result;
}