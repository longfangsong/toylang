#include "intLiteral.h"
#include <stdio.h>

static void generate_rvalue_code(IntLiteral *rValue) {

}

static char *rvalue_ir(IntLiteral *rValue) {
    char *result = malloc(16);
    sprintf(result, "%d", rValue->value);
    return result;
}

#ifdef DEBUG

static void print_ast_node(IntLiteral *node, size_t layer) {
    for (size_t i = 0; i < layer; ++i) {
        printf("  ");
    }
    printf("IntLiteral %d", node->value);
}

#endif

IntLiteral *create_int_literal(int value) {
    IntLiteral *result = (IntLiteral *) malloc(sizeof(IntLiteral));
    ((ASTNode *) result)->free_node = (void (*)(ASTNode *)) free;
#ifdef DEBUG
    ((Expression *) result)->print_ast_node = (void (*)(struct ASTNode *node, size_t layer)) print_ast_node;
#endif
    ((RValue *) result)->rvalue_ir = (char *(*)(RValue *)) rvalue_ir;
    ((RValue *) result)->generate_rvalue_code = (void (*)(RValue *)) generate_rvalue_code;
    result->value = value;
    return result;
}