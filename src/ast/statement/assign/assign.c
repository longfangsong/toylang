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

static void generate_code(AssignStatement *node) {
    ((LValue *) (node->lhs))->generate_lvalue_code((LValue *) (node->lhs));
    node->rhs->generate_rvalue_code(node->rhs);
    char *rvalue_ir = node->rhs->rvalue_ir(node->rhs);
    char *lvalue_ir = node->lhs->lvalue_ir(node->lhs);
    printf("store %s %s, %s* %s\n",
           type_string(((RValue *) (node->lhs))->type),
           rvalue_ir,
           type_string(((RValue *) (node->lhs))->type),
           lvalue_ir);
    free(rvalue_ir);
    free(lvalue_ir);
}

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
    ((ASTNode *) result)->generate_code = (void (*)(ASTNode *)) generate_code;
    result->lhs = lhs;
    result->rhs = rhs;
    return result;
}