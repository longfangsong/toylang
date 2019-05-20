#include "doubleLiteral.h"
#include <stdio.h>

static void generate_rvalue_code(DoubleLiteral *rValue) {

}

static char *rvalue_ir(DoubleLiteral *rValue) {
    char *result = malloc(64);
    sprintf(result, "%64.32lf", rValue->value);
    return result;
}

#ifdef DEBUG

static void print_ast_node(DoubleLiteral *node, size_t layer) {
    for (size_t i = 0; i < layer; ++i) {
        printf("  ");
    }
    printf("DoubleLiteral %g\n", node->value);
}

#endif

DoubleLiteral *create_double_literal(double value) {
    DoubleLiteral *result = (DoubleLiteral *) malloc(sizeof(DoubleLiteral));
    ((ASTNode *) result)->free_node = (void (*)(ASTNode *)) free;
#ifdef DEBUG
    ((Expression *) result)->print_ast_node = (void (*)(ASTNode *node, size_t layer)) print_ast_node;
#endif
    ((RValue *) result)->type = Double;
    ((RValue *) result)->rvalue_ir = (char *(*)(RValue *)) rvalue_ir;
    ((RValue *) result)->generate_rvalue_code = (void (*)(RValue *)) generate_rvalue_code;
    result->value = value;
    return result;
}