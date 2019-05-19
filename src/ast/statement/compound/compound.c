#include "compound.h"
#include <stdio.h>

size_t next_namespace_id = 0;

#ifdef DEBUG

static void print_ast_node(CompoundStatement *compoundStatement, size_t layer) {
    for (size_t i = 0; i < layer; ++i) {
        printf("  ");
    }
    printf("CompoundStatement %zu {\n", layer);
    for (size_t i = 0; i < compoundStatement->child_count; ++i) {
        ((ASTNode *) (compoundStatement->children[i]))->print_ast_node((ASTNode *) (compoundStatement->children[i]),
                                                                       layer + 1);
    }
    for (size_t i = 0; i < layer; ++i) {
        printf("  ");
    }
    printf("}\n");
}

#endif

static void free_node(CompoundStatement *compoundStatement) {
    for (size_t i = 0; i < compoundStatement->child_count; ++i) {
        ((ASTNode *) compoundStatement)->free_node((ASTNode *) compoundStatement);
    }
    free(compoundStatement);
}


CompoundStatement *create_compound_statement() {
    CompoundStatement *result = (CompoundStatement *) malloc(sizeof(CompoundStatement));
    result->child_count = 0;
    result->children = NULL;
    result->namespace_id = next_namespace_id++;
#ifdef DEBUG
    ((ASTNode *) (result))->print_ast_node = (void (*)(ASTNode *, size_t)) print_ast_node;
#endif
    ((ASTNode *) (result))->free_node = (void (*)(ASTNode *)) free_node;
    return result;
}

void add_statement(CompoundStatement *compoundStatement, Statement *statement) {
    compoundStatement->children = (Statement **) realloc(compoundStatement->children,
                                                         sizeof(Statement *) * (compoundStatement->child_count + 1));
    compoundStatement->children[compoundStatement->child_count] = statement;
    ++compoundStatement->child_count;
}