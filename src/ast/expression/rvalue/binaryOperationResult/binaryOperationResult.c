#include "binaryOperationResult.h"
#include <stdio.h>

#ifdef DEBUG

static void print_ast_node(BinaryOperationResult *node, size_t layer) {
    for (size_t i = 0; i < layer; ++i) {
        printf("  ");
    }
    printf("BinaryOperationResult ");
    switch (node->operator_id) {
        case '+':
        case '-':
        case '*':
        case '/':
        case '<':
        case '>':
            printf("%c", (char) (node->operator_id));
            break;
        default:
            exit(1);
    }
    printf("\n");
    for (size_t i = 0; i < layer; ++i) {
        printf("  ");
    }
    printf("lhs:\n");
    ((ASTNode *) node->lhs)->print_ast_node((ASTNode *) node->lhs, layer + 1);
    for (size_t i = 0; i < layer; ++i) {
        printf("  ");
    }
    printf("rhs:\n");
    ((ASTNode *) node->rhs)->print_ast_node((ASTNode *) node->rhs, layer + 1);
}

#endif

static void free_node(BinaryOperationResult *node) {
    ((ASTNode *) (node->lhs))->free_node((ASTNode *) (node->lhs));
    ((ASTNode *) (node->rhs))->free_node((ASTNode *) (node->rhs));
    free((ASTNode *) (node));
}

static char *rvalue_ir(BinaryOperationResult *rValue) {
    char *result = malloc(128);
    sprintf(result, "%%temp_%zu", rValue->temp_register_id);
    return result;
}

static void generate_rvalue_code(BinaryOperationResult *node) {
    node->lhs->generate_rvalue_code(node->lhs);
    node->rhs->generate_rvalue_code(node->rhs);
    printf("%s = ", ((RValue *) node)->rvalue_ir((RValue *) node));
    switch (node->operator_id) {
        case '+':
            if (((RValue *) node)->type == Double) {
                printf("fadd");
            } else {
                printf("add nsw");
            }
            break;
        case '-':
            if (((RValue *) node)->type == Double) {
                printf("fsub");
            } else {
                printf("sub nsw");
            }
            break;
        case '*':
            if (((RValue *) node)->type == Double) {
                printf("fmul");
            } else {
                printf("mul nsw");
            }
            break;
        case '/':
            if (((RValue *) node)->type == Double) {
                printf("fdif");
            } else {
                printf("sdif");
            }
            break;
        default:
            exit(1);
    }
    printf(" %s ", type_string(((RValue *) node)->type));
    printf("%s, %s\n", node->rhs->rvalue_ir(node->rhs), node->lhs->rvalue_ir(node->lhs));
}

BinaryOperationResult *create_binary_operation_result(size_t operator_id, RValue *lhs, RValue *rhs) {
    BinaryOperationResult *result = (BinaryOperationResult *) malloc(sizeof(BinaryOperationResult));
    ((ASTNode *) result)->free_node = (void (*)(ASTNode *)) free_node;
#ifdef DEBUG
    ((Expression *) result)->print_ast_node = (void (*)(struct ASTNode *node, size_t layer)) print_ast_node;
#endif
    if (lhs->type != rhs->type) {
        exit(1);
    }
    switch (result->operator_id) {
        case '>':
        case '<':
            ((RValue *) result)->type = Bool;
            break;
        default:
            ((RValue *) result)->type = lhs->type;
            break;
    }
    ((RValue *) result)->rvalue_ir = (char *(*)(struct RValue *)) rvalue_ir;
    ((RValue *) result)->generate_rvalue_code = (void (*)(RValue *)) generate_rvalue_code;
    result->operator_id = operator_id;
    result->lhs = lhs;
    result->rhs = rhs;
    result->temp_register_id = next_temp_register++;
    return result;
}