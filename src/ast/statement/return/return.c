#include "return.h"
#include <stdio.h>

static void free_node(ReturnStatement *node) {
    if (node->expression != NULL)
        ((ASTNode *) (node->expression))->free_node((ASTNode *) (node->expression));
    free(node);
}

#ifdef DEBUG

static void print_ast_node(ReturnStatement *node, size_t layer) {
    for (size_t i = 0; i < layer; ++i) {
        printf("  ");
    }
    printf("ReturnStatement\n");
    if (node->expression != NULL)
        ((ASTNode *) (node->expression))->print_ast_node((ASTNode *) (node->expression), layer + 1);
}

#endif

static void generate_code(ReturnStatement *node) {
    if (node->expression != NULL) {
        node->expression->generate_rvalue_code(node->expression);
        char *return_content_ir = node->expression->rvalue_ir(node->expression);
        printf("ret %s %s", type_string(node->expression->type), return_content_ir);
        free(return_content_ir);
    } else {
        printf("ret");
    }
}


ReturnStatement *create_return_statement(RValue *expression) {
    ReturnStatement *result = (ReturnStatement *) malloc(sizeof(ReturnStatement));
#ifdef DEBUG
    ((ASTNode *) result)->print_ast_node = (void (*)(ASTNode *, size_t)) print_ast_node;
#endif
    ((ASTNode *) result)->free_node = (void (*)(ASTNode *)) free;
    ((Statement *) result)->generate_code = (void (*)(Statement *)) generate_code;
    result->expression = expression;
    return result;
}