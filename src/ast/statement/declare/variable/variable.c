#include "variable.h"
#include <stdio.h>

static void free_node(VariableDeclareStatement *node) {
    ((ASTNode *) (node->initial))->free_node((ASTNode *) (node->initial));
    free(node);
}

#ifdef DEBUG

static void print_ast_node(VariableDeclareStatement *node, size_t layer) {
    for (size_t i = 0; i < layer; ++i) {
        printf("  ");
    }
    printf("VariableDeclareStatement %s %s\n", symbol_type(node->variable), node->variable->name);
    if (node->initial != NULL) {
        ((ASTNode *) node->initial)->print_ast_node((ASTNode *) node->initial, layer + 1);
    }
}

#endif

static void generate_code(VariableDeclareStatement *node) {
    printf("%%%s_%zu = alloca %s\n", node->variable->name, node->variable->namespace_id, symbol_type(node->variable));
    if (node->initial != NULL) {
        ((Statement *) (node->initial))->generate_code((Statement *) (node->initial));
    }
}

VariableDeclareStatement *create_variable_declare_statement(Symbol *variable, AssignStatement *initial) {
    VariableDeclareStatement *result = (VariableDeclareStatement *) malloc(sizeof(VariableDeclareStatement));
#ifdef DEBUG
    ((ASTNode *) result)->print_ast_node = (void (*)(ASTNode *, size_t)) print_ast_node;
#endif
    ((ASTNode *) result)->free_node = (void (*)(ASTNode *)) free;
    ((Statement *) result)->generate_code = (void (*)(Statement *)) generate_code;
    result->variable = variable;
    result->initial = initial;
    return result;
}