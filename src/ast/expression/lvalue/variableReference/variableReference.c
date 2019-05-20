#include "variableReference.h"
#include <stdio.h>


#ifdef DEBUG

static void print_ast_node(VariableReference *node, size_t layer) {
    for (size_t i = 0; i < layer; ++i) {
        printf("  ");
    }
    printf("VariableReference: %s\n", node->variable->name);
}

#endif

static char *lvalue_ir(VariableReference *rValue) {
    char *result = malloc(128);
    sprintf(result, "%%%s_%zu", rValue->variable->name, rValue->variable->layer);
    return result;
}

static void generate_rvalue_code(VariableReference *rValue) {
    char *rvalue_ir_string = ((RValue *) rValue)->rvalue_ir((RValue *) rValue);
    char *lvalue_ir_string = ((LValue *) rValue)->lvalue_ir((LValue *) rValue);
    printf("%s = load %s, %s* %s",
           rvalue_ir_string,
           type_name(rValue->variable),
           type_name(rValue->variable),
           lvalue_ir_string);
    free(rvalue_ir_string);
    free(lvalue_ir_string);
}

VariableReference *create_variable_reference(Symbol *variable) {
    VariableReference *result = (VariableReference *) malloc(sizeof(VariableReference));
    ((ASTNode *) result)->free_node = (void (*)(ASTNode *)) free;
#ifdef DEBUG
    ((ASTNode *) result)->print_ast_node = (void (*)(ASTNode *, size_t)) print_ast_node;
#endif
    ((RValue *) result)->generate_rvalue_code = (void (*)(RValue *)) generate_rvalue_code;
    ((LValue *) result)->lvalue_ir = (char *(*)(LValue *)) lvalue_ir;
    result->variable = variable;
    ((RValue *) result)->type = variable->type;
    return result;
}