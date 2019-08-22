#include "print.h"
#include <stdio.h>

static void free_node(PrintStatement *node) {
    ((ASTNode *) (node->expression))->free_node((ASTNode *) (node->expression));
    free(node);
}

#ifdef DEBUG

static void print_ast_node(PrintStatement *node, size_t layer) {
    for (size_t i = 0; i < layer; ++i) {
        printf("  ");
    }
    printf("PrintStatement\n");
}

#endif

static void generate_code(PrintStatement *node) {
    node->expression->generate_rvalue_code(node->expression);
    char *rvalue_ir = node->expression->rvalue_ir(node->expression);
    switch (node->expression->type) {
        case Int:
            printf("call i32 (i8*, ...) @printf(i8* getelementptr inbounds "
                   "([4 x i8], [4 x i8]* @int_fmt_str, i32 0, i32 0), i32 %s)\n",
                   rvalue_ir);
            break;
        case Double:
            printf("call i32 (i8*, ...) @printf(i8* getelementptr inbounds "
                   "([4 x i8], [4 x i8]* @double_fmt_str, i32 0, i32 0), double %s)\n",
                   rvalue_ir);
            break;
        default:
            break;
    }
}

PrintStatement *create_print_statement(RValue *expression) {
    PrintStatement *result = (PrintStatement *) malloc(sizeof(PrintStatement));
#ifdef DEBUG
    ((ASTNode *) result)->print_ast_node = (void (*)(ASTNode *, size_t)) print_ast_node;
#endif
    ((ASTNode *) result)->free_node = (void (*)(ASTNode *)) free_node;
    ((Statement *) result)->generate_code = (void (*)(Statement *)) generate_code;
    result->expression = expression;
    return result;
}