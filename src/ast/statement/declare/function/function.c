#include "function.h"
#include <stdio.h>

static void print_ast_node(FunctionDeclareStatement *node, size_t layer) {
    for (size_t i = 0; i < layer; ++i) {
        printf("  ");
    }
    printf("FunctionDeclareStatement %s\n", ((Symbol *) (node->function))->name);
    for (size_t j = 0; j < layer + 1; ++j) {
        printf("  ");
    }
    printf("Params:\n");
    for (size_t i = 0; i < node->function->params->length; ++i) {
        for (size_t j = 0; j < layer + 1; ++j) {
            printf("  ");
        }
        printf("%s: %s\n", node->function->params->content[i]->name, symbol_type(node->function->params->content[i]));
    }
    for (size_t j = 0; j < layer + 1; ++j) {
        printf("  ");
    }
    printf("Statement:");
    ((ASTNode *) (node->function->statement))->print_ast_node((ASTNode *) (node->function->statement), layer + 1);
}

static void generate_code(FunctionDeclareStatement *node) {
    printf("define %s @%s(", symbol_type((Symbol *) (node->function)), ((Symbol *) (node->function))->name);
    for (size_t i = 0; i < node->function->params->length; ++i) {
        if (i != 0) {
            printf(",");
        }
        printf("%s %%%s", symbol_type(node->function->params->content[i]), node->function->params->content[i]->name);
    }
    printf(") {");
    ((Statement *) (node->function->statement))->generate_code((Statement *) (node->function->statement));
    printf("}");
}

FunctionDeclareStatement *create_function_declare(Function *function) {
    FunctionDeclareStatement *result = (FunctionDeclareStatement *) malloc(sizeof(FunctionDeclareStatement));
    ((ASTNode *) result)->free_node = (void (*)(ASTNode *)) free;
    ((ASTNode *) result)->print_ast_node = (void (*)(ASTNode *, size_t)) print_ast_node;
    ((Statement *) result)->generate_code = (void (*)(Statement *)) generate_code;
    result->function = function;
    return result;
}
