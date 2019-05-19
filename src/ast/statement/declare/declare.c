#include "declare.h"
#include <stdio.h>

static void free_node(DeclareStatement *node) {
    ((ASTNode *) (node->initial))->free_node((ASTNode *) (node->initial));
    free(node);
}

static void print_ast_node(DeclareStatement *node, size_t layer) {
    for (size_t i = 0; i < layer; ++i) {
        printf("  ");
    }
    printf("DeclareStatement %s %s\n", type_name(node->variable), node->variable->name);
    if (node->initial != NULL) {
        ((ASTNode *) node->initial)->print_ast_node((ASTNode *) node->initial, layer + 1);
    }
}

DeclareStatement *create_declare_statement(Symbol *variable, AssignStatement *initial) {
    DeclareStatement *result = (DeclareStatement *) malloc(sizeof(DeclareStatement));
#ifdef DEBUG
    ((ASTNode *) result)->print_ast_node = (void (*)(ASTNode *, size_t)) print_ast_node;
#endif
    ((ASTNode *) result)->free_node = (void (*)(ASTNode *)) free;
    result->variable = variable;
    result->initial = initial;
    return result;
}